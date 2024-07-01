use crate::{LanguageModel, Message};
use anyhow::{anyhow, Result};
use futures::{channel::mpsc, future::BoxFuture, FutureExt, SinkExt, StreamExt};
use gpui::BackgroundExecutor;
use reqwest::Client;
use serde::Deserialize;

pub struct HuggingFaceClient {
    client: Client,
    endpoint: String,
    api_key: String,
    background_executor: BackgroundExecutor,
}

impl HuggingFaceClient {
    pub fn new(endpoint: String, api_key: String, background_executor: BackgroundExecutor) -> Self {
        Self {
            client: Client::new(),
            endpoint,
            api_key,
            background_executor,
        }
    }
}

impl LanguageModel for HuggingFaceClient {
    fn stream_completion(
        &self,
        messages: Vec<Message>,
    ) -> BoxFuture<Result<mpsc::Receiver<String>>> {
        async move {
            let (mut tx, rx) = mpsc::channel(100);

            let mut inputs = messages
                .iter()
                .map(|msg| format!("<|im_start|>{}\n{}<|im_end|>", msg.role, msg.content))
                .collect::<Vec<String>>()
                .join("\n");
            inputs.push_str("<|im_end|>");
            inputs.push_str("<|im_start|>assistant\n");

            let request = serde_json::json!({
                "inputs": inputs,
                "stream": true,
                "max_tokens": 2048
            });

            let response = self
                .client
                .post(&self.endpoint)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .json(&request)
                .send()
                .await?;

            if !response.status().is_success() {
                return Err(anyhow!(
                    "error streaming completion: {:?}",
                    response.text().await?
                ));
            }

            self.background_executor
                .spawn(async move {
                    let mut stream = response.bytes_stream();
                    while let Some(chunk) = stream.next().await {
                        if let Ok(chunk) = chunk {
                            if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                                for line in text.lines() {
                                    if line.starts_with("data:") {
                                        let json_str = line.trim_start_matches("data:");

                                        if let Ok(output) =
                                            serde_json::from_str::<StreamOutput>(json_str)
                                        {
                                            if !output.token.special {
                                                let _ = tx.send(output.token.text).await;
                                            }
                                        }
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

#[derive(Debug, Deserialize)]
struct StreamOutput {
    index: u32,
    token: Token,
    generated_text: Option<String>,
    details: Option<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
struct Token {
    id: u32,
    text: String,
    logprob: f64,
    special: bool,
}
