use anyhow::Result;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;

#[derive(Debug, Serialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatCompletionChunk {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    delta: Delta,
}

#[derive(Debug, Deserialize)]
struct Delta {
    content: Option<String>,
}

pub struct GroqClient {
    client: Client,
    api_key: String,
}

impl GroqClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Client::new(),
            api_key,
        }
    }

    pub async fn stream_completion(
        &self,
        model: String,
        messages: Vec<Message>,
    ) -> Result<mpsc::Receiver<String>> {
        let (tx, rx) = mpsc::channel(100);

        let request = ChatCompletionRequest {
            model,
            messages,
            stream: true,
        };

        let response = self
            .client
            .post("https://api.groq.com/openai/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(&request)
            .send()
            .await?;

        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                if let Ok(chunk) = chunk {
                    if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                        for line in text.lines() {
                            if line.starts_with("data: ") && line != "data: [DONE]" {
                                if let Ok(mut chunk) =
                                    serde_json::from_str::<ChatCompletionChunk>(&line[6..])
                                {
                                    if let Some(content) =
                                        chunk.choices.pop().unwrap().delta.content
                                    {
                                        let _ = tx.send(content).await;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        Ok(rx)
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let api_key = std::env::var("GROQ_API_KEY").expect("GROQ_API_KEY must be set");
    let client = GroqClient::new(api_key);

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are a helpful assistant.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: "Tell me a short joke about programming.".to_string(),
        },
    ];

    let mut receiver = client
        .stream_completion("mixtral-8x7b-32768".to_string(), messages)
        .await?;

    while let Some(content) = receiver.recv().await {
        print!("{}", content);
    }

    println!();
    Ok(())
}
