use anyhow::{anyhow, Result};
use futures::StreamExt;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::Serialize;
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    path::{Path, PathBuf},
    sync::Arc,
};
use tokenizers::tokenizer::Tokenizer;
use tokenizers::FromPretrainedParameters;
use tokio::sync::mpsc;
use tokio::sync::Mutex;

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

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

const CHUNK_SIZE: usize = 16_000;
const OVERLAP: usize = 2_000;

#[derive(Debug)]
enum Entry {
    File(PathBuf),
    Directory(PathBuf),
}

async fn summarize_project(root: &Path, num_workers: usize) -> Result<BTreeMap<PathBuf, String>> {
    let tokenizer = Tokenizer::from_pretrained(
        "Qwen/Qwen2-0.5B",
        Some(FromPretrainedParameters {
            revision: "main".into(),
            user_agent: HashMap::default(),
            auth_token: Some(
                std::env::var("HUGGINGFACE_API_TOKEN").expect("HUGGINGFACE_API_TOKEN not set"),
            ),
        }),
    )
    .unwrap();
    let client = Arc::new(OllamaClient::new("http://localhost:11434".into()));
    let queue = Arc::new(Mutex::new(VecDeque::new()));

    let multi_progress = Arc::new(MultiProgress::new());
    let overall_progress = multi_progress.add(ProgressBar::new_spinner());
    overall_progress.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap(),
    );
    overall_progress.set_message("Summarizing project...");

    // Populate the queue with files and directories
    let mut walker = ignore::WalkBuilder::new(root)
        .hidden(true)
        .ignore(true)
        .build();
    while let Some(entry) = walker.next() {
        if let Ok(entry) = entry {
            let path = entry.path().to_owned();
            if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                queue.lock().await.push_back(Entry::Directory(path));
            } else {
                queue.lock().await.push_back(Entry::File(path));
            }
        }
    }

    let total_entries = queue.lock().await.len();
    let progress_bar = multi_progress.add(ProgressBar::new(total_entries as u64));
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("##-"),
    );

    let summaries = Arc::new(Mutex::new(BTreeMap::new()));

    let workers: Vec<_> = (0..num_workers)
        .map(|_| {
            let queue = Arc::clone(&queue);
            let client = Arc::clone(&client);
            let summaries = Arc::clone(&summaries);
            let tokenizer = tokenizer.clone();
            let progress_bar = progress_bar.clone();

            tokio::spawn(async move {
                loop {
                    let mut queue_lock = queue.lock().await;
                    let Some(entry) = queue_lock.pop_front() else {
                        break;
                    };

                    match entry {
                        Entry::File(path) => {
                            drop(queue_lock);
                            let summary = async {
                                let content = tokio::fs::read_to_string(&path).await?;
                                let chunks =
                                    split_into_chunks(&content, &tokenizer, CHUNK_SIZE, OVERLAP);
                                let chunk_summaries = summarize_chunks(&client, &chunks).await?;
                                combine_summaries(&client, &chunk_summaries, true).await
                            };

                            let summary = summary
                                .await
                                .unwrap_or_else(|_| "path could not be summarized".into());
                            summaries.lock().await.insert(path, summary);
                            progress_bar.inc(1);
                        }
                        Entry::Directory(path) => {
                            let mut dir_summaries = Vec::new();
                            let mut all_children_summarized = true;
                            let dir_walker = ignore::WalkBuilder::new(&path)
                                .hidden(true)
                                .ignore(true)
                                .max_depth(Some(1))
                                .build();
                            for entry in dir_walker {
                                if let Ok(entry) = entry {
                                    if entry.path() != path {
                                        if let Some(summary) =
                                            summaries.lock().await.get(entry.path())
                                        {
                                            dir_summaries.push(summary.clone());
                                        } else {
                                            all_children_summarized = false;
                                            break;
                                        }
                                    }
                                }
                            }
                            if all_children_summarized {
                                drop(queue_lock);
                                let combined_summary =
                                    combine_summaries(&client, &dir_summaries, false).await?;
                                summaries.lock().await.insert(path, combined_summary);
                                progress_bar.inc(1);
                            } else {
                                queue_lock.push_back(Entry::Directory(path));
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

    progress_bar.finish_with_message("Summarization complete");
    overall_progress.finish_with_message("Project summarization finished");

    Ok(Arc::try_unwrap(summaries).unwrap().into_inner())
}

fn split_into_chunks(
    content: &str,
    tokenizer: &Tokenizer,
    chunk_size: usize,
    overlap: usize,
) -> Vec<String> {
    let mut chunks = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut current_chunk = String::new();
    let mut current_tokens = 0;

    for line in lines {
        let line_tokens = tokenizer.encode(line, false).unwrap().get_ids().len();
        if current_tokens + line_tokens > chunk_size {
            chunks.push(current_chunk.clone());
            current_chunk.clear();
            current_tokens = 0;
        }
        current_chunk.push_str(line);
        current_chunk.push('\n');
        current_tokens += line_tokens;
    }

    if !current_chunk.is_empty() {
        chunks.push(current_chunk);
    }

    // Add overlap
    for i in 1..chunks.len() {
        let overlap_text = chunks[i - 1]
            .lines()
            .rev()
            .take(overlap)
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join("\n");
        chunks[i] = format!("{}\n{}", overlap_text, chunks[i]);
    }

    chunks
}

async fn summarize_chunks(client: &OllamaClient, chunks: &[String]) -> Result<Vec<String>> {
    let mut chunk_summaries = Vec::new();

    for chunk in chunks {
        let summary = summarize_file(client, chunk).await?;
        chunk_summaries.push(summary);
    }

    Ok(chunk_summaries)
}

async fn summarize_file(client: &OllamaClient, content: &str) -> Result<String> {
    let messages = vec![
        Message {
            role: "system".to_string(),
            content:
                "You are a code summarization assistant. Provide a brief summary of the given code chunk, focusing on its main functionality and purpose.".to_string(),
        },
        Message {
            role: "user".to_string(),
            content: content.to_string(),
        },
    ];

    let mut receiver = client
        .stream_completion("qwen2:0.5b".to_string(), messages)
        .await?;

    let mut summary = String::new();
    while let Some(content) = receiver.recv().await {
        summary.push_str(&content);
    }

    Ok(summary)
}

async fn combine_summaries(
    client: &OllamaClient,
    summaries: &[String],
    is_chunk: bool,
) -> Result<String> {
    let combined_content = summaries.join("\n\n");
    let prompt = if is_chunk {
        "You are a code summarization assistant. Combine the given summaries into a single, coherent summary that captures the overall functionality and structure of the code. Ensure that the final summary is comprehensive and reflects the content as if it was summarized from a single, complete file."
    } else {
        "You are a code summarization assistant. Combine the given summaries of different files or directories into a single, coherent summary that captures the overall structure and functionality of the project or directory. Focus on the relationships between different components and the high-level architecture."
    };

    let messages = vec![
        Message {
            role: "system".to_string(),
            content: prompt.to_string(),
        },
        Message {
            role: "user".to_string(),
            content: combined_content,
        },
    ];

    let mut receiver = client
        .stream_completion("qwen2:0.5b".to_string(), messages)
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
    println!("Project Summary:\n{:?}", summary);

    Ok(())
}

// #[derive(Debug, Serialize)]
// struct ChatCompletionRequest {
//     model: String,
//     messages: Vec<Message>,
//     stream: bool,
// }
//
// #[derive(Debug, Deserialize)]
// struct ChatCompletionChunk {
//     choices: Vec<Choice>,
// }

// #[derive(Debug, Deserialize)]
// struct Choice {
//     delta: Delta,
// }

// #[derive(Debug, Deserialize)]
// struct Delta {
//     content: Option<String>,
// }

// pub struct GroqClient {
//     client: Client,
//     api_key: String,
// }

// impl GroqClient {
//     pub fn new(api_key: String) -> Self {
//         Self {
//             client: Client::new(),
//             api_key,
//         }
//     }

//     async fn stream_completion(
//         &self,
//         model: String,
//         messages: Vec<Message>,
//     ) -> Result<mpsc::Receiver<String>> {
//         let (tx, rx) = mpsc::channel(100);

//         let request = ChatCompletionRequest {
//             model,
//             messages,
//             stream: true,
//         };

//         let response = self
//             .client
//             .post("https://api.groq.com/openai/v1/chat/completions")
//             .header("Authorization", format!("Bearer {}", self.api_key))
//             .json(&request)
//             .send()
//             .await?;

//         if !response.status().is_success() {
//             return Err(anyhow!(
//                 "error streaming completion: {:?}",
//                 response.text().await?
//             ));
//         }

//         tokio::spawn(async move {
//             let mut stream = response.bytes_stream();
//             while let Some(chunk) = stream.next().await {
//                 if let Ok(chunk) = chunk {
//                     if let Ok(text) = String::from_utf8(chunk.to_vec()) {
//                         for line in text.lines() {
//                             if line.starts_with("data: ") && line != "data: [DONE]" {
//                                 if let Ok(mut chunk) =
//                                     serde_json::from_str::<ChatCompletionChunk>(&line[6..])
//                                 {
//                                     if let Some(content) =
//                                         chunk.choices.pop().and_then(|choice| choice.delta.content)
//                                     {
//                                         let _ = tx.send(content).await;
//                                     }
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//         });

//         Ok(rx)
//     }
// }
