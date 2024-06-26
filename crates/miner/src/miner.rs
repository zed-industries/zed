use anyhow::Result;
use futures::{Stream, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokenizers::tokenizer::Tokenizer;
use tokenizers::FromPretrainedParameters;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use walkdir::WalkDir;

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

// Below we perform a bottom up traversal over each worktree in the project.
// We push an entry for each file and directory into a queue.
// We then read from that queue with N workers in a tokio thread pool.
// For each file, we perform a summarization with a prompt.
// For each directory, we combine the summaries of all its files with a prompt.
// If a file is too big, truncate it at the max tokens of mixtral model, 32k.
// Use the tokenizers crate to estimate token counts

const MAX_TOKENS: usize = 32_000;

#[derive(Debug)]
enum Entry {
    File(PathBuf),
    Directory(PathBuf),
}

async fn summarize_project(root: &Path, num_workers: usize) -> Result<String> {
    let tokenizer = Tokenizer::from_pretrained(
        "mistralai/Mixtral-8x7B-v0.1",
        Some(FromPretrainedParameters {
            revision: String::new(),
            user_agent: HashMap::default(),
            auth_token: Some(
                std::env::var("HUGGINGFACE_API_TOKEN").expect("HUGGINGFACE_API_TOKEN not set"),
            ),
        }),
    )
    .unwrap();
    let client = Arc::new(GroqClient::new(std::env::var("GROQ_API_KEY")?));
    let queue = Arc::new(Mutex::new(VecDeque::new()));

    // Populate the queue with files and directories
    for entry in WalkDir::new(root)
        .min_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path().to_owned();
        if entry.file_type().is_dir() {
            queue.lock().await.push_back(Entry::Directory(path));
        } else {
            queue.lock().await.push_back(Entry::File(path));
        }
    }

    let summaries = Arc::new(Mutex::new(HashMap::new()));

    let workers: Vec<_> = (0..num_workers)
        .map(|_| {
            let queue = Arc::clone(&queue);
            let client = Arc::clone(&client);
            let summaries = Arc::clone(&summaries);
            let tokenizer = tokenizer.clone();

            tokio::spawn(async move {
                while let Some(entry) = queue.lock().await.pop_front() {
                    match entry {
                        Entry::File(path) => {
                            let content = tokio::fs::read_to_string(&path).await?;
                            let truncated_content =
                                truncate_to_max_tokens(&content, &tokenizer, MAX_TOKENS);
                            let summary = summarize_file(&client, &truncated_content).await?;
                            summaries.lock().await.insert(path, summary);
                        }
                        Entry::Directory(path) => {
                            let mut dir_summaries = Vec::new();
                            let mut all_children_summarized = true;
                            for entry in path.read_dir()? {
                                if let Ok(entry) = entry {
                                    if let Some(summary) = summaries.lock().await.get(&entry.path())
                                    {
                                        dir_summaries.push(summary.clone());
                                    } else {
                                        all_children_summarized = false;
                                        break;
                                    }
                                }
                            }
                            if all_children_summarized {
                                let combined_summary =
                                    combine_summaries(&client, &dir_summaries).await?;
                                summaries.lock().await.insert(path, combined_summary);
                            } else {
                                queue.lock().await.push_back(Entry::Directory(path));
                            }
                        }
                    }
                }
                Ok::<_, anyhow::Error>(())
            })
        })
        .collect();

    for worker in workers {
        worker.await??;
    }

    let summaries = summaries.lock().await;
    Ok(summaries.get(root).cloned().unwrap_or_default())
}

fn truncate_to_max_tokens(content: &str, tokenizer: &Tokenizer, max_tokens: usize) -> String {
    let encoding = tokenizer.encode(content, false).unwrap();
    if encoding.get_ids().len() <= max_tokens {
        content.to_string()
    } else {
        tokenizer
            .decode(&encoding.get_ids()[..max_tokens], false)
            .unwrap()
    }
}

async fn summarize_file(client: &GroqClient, content: &str) -> Result<String> {
    let messages = vec![
        Message {
            role: "system".to_string(),
            content:
                "You are a code summarization assistant. Provide a brief summary of the given code."
                    .to_string(),
        },
        Message {
            role: "user".to_string(),
            content: content.to_string(),
        },
    ];

    let mut receiver = client
        .stream_completion("mixtral-8x7b-32768".to_string(), messages)
        .await?;

    let mut summary = String::new();
    while let Some(content) = receiver.recv().await {
        summary.push_str(&content);
    }

    Ok(summary)
}

async fn combine_summaries(client: &GroqClient, summaries: &[String]) -> Result<String> {
    let combined_content = summaries.join("\n\n");
    let messages = vec![
        Message {
            role: "system".to_string(),
            content: "You are a code summarization assistant. Combine the given summaries into a single, coherent summary.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: combined_content,
        },
    ];

    let mut receiver = client
        .stream_completion("mixtral-8x7b-32768".to_string(), messages)
        .await?;

    let mut combined_summary = String::new();
    while let Some(content) = receiver.recv().await {
        combined_summary.push_str(&content);
    }

    Ok(combined_summary)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <project_path>", args[0]);
        std::process::exit(1);
    }

    let project_path = Path::new(&args[1]);
    if !project_path.exists() || !project_path.is_dir() {
        eprintln!("Error: The provided path does not exist or is not a directory.");
        std::process::exit(1);
    }

    println!("Summarizing project at: {}", project_path.display());
    let summary = summarize_project(project_path, 16).await?;
    println!("Project Summary:\n{}", summary);

    Ok(())
}
