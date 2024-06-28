#![allow(unused)]

use anyhow::{anyhow, Result};
use futures::StreamExt;
use reqwest::Client;
use tokio::sync::mpsc;

use crate::Message;

pub struct OllamaClient {
    client: Client,
    base_url: String,
}

impl OllamaClient {
    pub fn new(base_url: String) -> Self {
        Self {
            client: Client::new(),
            base_url,
        }
    }

    async fn stream_completion(
        &self,
        model: String,
        messages: Vec<Message>,
    ) -> Result<mpsc::Receiver<String>> {
        let (tx, rx) = mpsc::channel(100);

        let request = serde_json::json!({
            "model": model,
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

        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                if let Ok(chunk) = chunk {
                    if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                        if let Ok(response) = serde_json::from_str::<serde_json::Value>(&text) {
                            if let Some(content) = response["message"]["content"].as_str() {
                                let _ = tx.send(content.to_string()).await;
                            }
                        }
                    }
                }
            }
        });

        Ok(rx)
    }
}
