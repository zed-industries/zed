use crate::{LanguageModel, Message};
use anyhow::{anyhow, Result};
use futures::{
    channel::mpsc, future::BoxFuture, io::BufReader, AsyncBufReadExt, AsyncReadExt, FutureExt,
    SinkExt, StreamExt,
};
use gpui::BackgroundExecutor;
use http::HttpClient;
use serde::Deserialize;
use std::sync::Arc;

pub struct HuggingFaceClient {
    client: Arc<dyn HttpClient>,
    endpoint: String,
    api_key: String,
    background_executor: BackgroundExecutor,
}

impl HuggingFaceClient {
    pub fn new(endpoint: String, api_key: String, background_executor: BackgroundExecutor) -> Self {
        Self {
            client: http::client(None),
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

            let request = http::Request::builder()
                .method(http::Method::POST)
                .uri(&self.endpoint)
                .header("Authorization", format!("Bearer {}", self.api_key))
                .header("Content-Type", "application/json")
                .body(http::AsyncBody::from(serde_json::to_vec(&request)?))?;

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
                    Ok(line) => {
                        let line = line.strip_prefix("data: ")?;
                        match serde_json::from_str::<StreamOutput>(line) {
                            Ok(output) => {
                                if !output.token.special {
                                    Some(Ok(output.token.text))
                                } else {
                                    None
                                }
                            }
                            Err(error) => Some(Err(anyhow!(error))),
                        }
                    }
                    Err(error) => Some(Err(anyhow!(error))),
                }
            });

            self.background_executor
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
