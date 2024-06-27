use anyhow::{anyhow, Result};
use futures::StreamExt;
use heed::{
    types::{SerdeJson, Str},
    Database as HeedDatabase, EnvOpenOptions, RwTxn,
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
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

pub struct HuggingFaceClient {
    client: Client,
    endpoint: String,
    api_key: String,
}

impl HuggingFaceClient {
    pub fn new(endpoint: String, api_key: String) -> Self {
        Self {
            client: Client::new(),
            endpoint,
            api_key,
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

        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            while let Some(chunk) = stream.next().await {
                if let Ok(chunk) = chunk {
                    if let Ok(text) = String::from_utf8(chunk.to_vec()) {
                        for line in text.lines() {
                            if line.starts_with("data:") {
                                let json_str = line.trim_start_matches("data:");
                                if json_str == "[DONE]" {
                                    break;
                                }

                                if let Ok(response) =
                                    serde_json::from_str::<serde_json::Value>(json_str)
                                {
                                    if let Some(content) =
                                        response["choices"][0]["delta"]["content"].as_str()
                                    {
                                        let _ = tx.send(content.to_string()).await;
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

const CHUNK_SIZE: usize = 5000;
const OVERLAP: usize = 2_000;

#[derive(Debug)]
enum Entry {
    File(PathBuf),
    Directory(PathBuf),
}

#[derive(Debug, Serialize, Deserialize)]
struct CachedSummary {
    summary: String,
    mtime: SystemTime,
}

#[derive(Clone)]
struct Database {
    tx: mpsc::Sender<Box<dyn FnOnce(&HeedDatabase<Str, SerdeJson<CachedSummary>>, RwTxn) + Send>>,
}

impl Database {
    async fn new(db_path: &Path, root: &Path) -> Result<Self> {
        std::fs::create_dir_all(&db_path)?;
        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(1024 * 1024 * 1024)
                .max_dbs(3000)
                .open(db_path)?
        };
        let mut wtxn = env.write_txn()?;
        let db_name = format!("summaries_{}", root.to_string_lossy());
        let db: HeedDatabase<Str, SerdeJson<CachedSummary>> =
            env.create_database(&mut wtxn, Some(&db_name))?;
        wtxn.commit()?;

        let (tx, mut rx) = mpsc::channel::<
            Box<dyn FnOnce(&HeedDatabase<Str, SerdeJson<CachedSummary>>, RwTxn) + Send>,
        >(100);

        tokio::spawn(async move {
            while let Some(f) = rx.recv().await {
                let wtxn = env.write_txn().unwrap();
                f(&db, wtxn);
            }
        });

        Ok(Self { tx })
    }

    async fn transact<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&HeedDatabase<Str, SerdeJson<CachedSummary>>, RwTxn) -> Result<T>
            + Send
            + 'static,
        T: 'static + Send,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.tx
            .send(Box::new(move |db, txn| {
                let result = f(db, txn);
                let _ = tx.send(result);
            }))
            .await
            .map_err(|_| anyhow!("database closed"))?;
        Ok(rx.await.map_err(|_| anyhow!("transaction failed"))??)
    }
}

async fn summarize_project(
    db_path: &Path,
    root: &Path,
    num_workers: usize,
) -> Result<BTreeMap<PathBuf, String>> {
    let database = Database::new(db_path, root).await?;

    let tokenizer = Tokenizer::from_pretrained(
        "mistralai/Mistral-7B-Instruct-v0.1",
        Some(FromPretrainedParameters {
            revision: "main".into(),
            user_agent: HashMap::default(),
            auth_token: Some(
                std::env::var("HUGGINGFACE_API_TOKEN").expect("HUGGINGFACE_API_TOKEN not set"),
            ),
        }),
    )
    .unwrap();
    let client = Arc::new(HuggingFaceClient::new(
        "https://c0es55wrh8muqy3g.us-east-1.aws.endpoints.huggingface.cloud/v1/chat/completions"
            .into(),
        std::env::var("HUGGINGFACE_API_TOKEN").expect("HUGGINGFACE_API_TOKEN not set"),
    ));
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
    let paths_loaded_from_cache = Arc::new(Mutex::new(BTreeMap::new()));

    let workers: Vec<_> = (0..num_workers)
        .map(|_| {
            let queue = Arc::clone(&queue);
            let client = Arc::clone(&client);
            let summaries = Arc::clone(&summaries);
            let tokenizer = tokenizer.clone();
            let progress_bar = progress_bar.clone();
            let database = database.clone();
            let paths_loaded_from_cache = Arc::clone(&paths_loaded_from_cache);

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
                                let mtime = tokio::fs::metadata(&path).await?.modified()?;
                                let key = path.to_string_lossy().to_string();

                                let cached = database
                                    .transact({
                                        let key = key.clone();
                                        move |db, txn| Ok(db.get(&txn, &key)?)
                                    })
                                    .await?;
                                if let Some(cached) = cached {
                                    if cached.mtime == mtime {
                                        paths_loaded_from_cache
                                            .lock()
                                            .await
                                            .insert(path.clone(), true);
                                        return Ok(cached.summary);
                                    }
                                }

                                progress_bar.set_message(format!("Summarizing {}", path.display()));

                                let content = tokio::fs::read_to_string(&path)
                                    .await
                                    .unwrap_or_else(|_| "binary file".into());
                                let chunks =
                                    split_into_chunks(&content, &tokenizer, CHUNK_SIZE, OVERLAP);
                                let chunk_summaries = summarize_chunks(&client, &chunks).await?;
                                let summary =
                                    combine_summaries(&client, &chunk_summaries, true).await?;

                                let cached_summary = CachedSummary {
                                    summary: summary.clone(),
                                    mtime,
                                };
                                database
                                    .transact(move |db, mut txn| {
                                        db.put(&mut txn, &key, &cached_summary)?;
                                        txn.commit()?;
                                        Ok(())
                                    })
                                    .await?;

                                anyhow::Ok(summary)
                            };

                            let summary = summary.await.unwrap_or_else(|error| {
                                format!("path could not be summarized: {error:?}")
                            });
                            summaries.lock().await.insert(path, summary);
                            progress_bar.inc(1);
                        }
                        Entry::Directory(path) => {
                            let mut dir_summaries = Vec::new();
                            let mut all_children_summarized = true;
                            let mut all_children_from_cache = true;
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
                                            if !paths_loaded_from_cache
                                                .lock()
                                                .await
                                                .get(entry.path())
                                                .unwrap_or(&false)
                                            {
                                                all_children_from_cache = false;
                                            }
                                        } else {
                                            all_children_summarized = false;
                                            break;
                                        }
                                    }
                                }
                            }
                            if all_children_summarized {
                                drop(queue_lock);

                                let combined_summary = async {
                                    let key = path.to_string_lossy().to_string();
                                    let mtime = tokio::fs::metadata(&path).await?.modified()?;

                                    if all_children_from_cache {
                                        if let Some(cached) = database
                                            .transact({
                                                let key = key.clone();
                                                move |db, txn| Ok(db.get(&txn, &key)?)
                                            })
                                            .await?
                                        {
                                            paths_loaded_from_cache
                                                .lock()
                                                .await
                                                .insert(path.clone(), true);
                                            return Ok(cached.summary);
                                        }
                                    }

                                    progress_bar
                                        .set_message(format!("Summarizing {}", path.display()));

                                    let combined_summary =
                                        combine_summaries(&client, &dir_summaries, false).await?;
                                    let cached_summary = CachedSummary {
                                        summary: combined_summary.clone(),
                                        mtime,
                                    };
                                    database
                                        .transact(move |db, mut txn| {
                                            db.put(&mut txn, &key, &cached_summary)?;
                                            txn.commit()?;
                                            Ok(())
                                        })
                                        .await?;
                                    anyhow::Ok(combined_summary)
                                };

                                let combined_summary = combined_summary
                                    .await
                                    .unwrap_or_else(|_| "could not combine summaries".into());
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

    // Remove deleted entries from the database
    database
        .transact(|db, mut txn| {
            let mut paths_to_delete = Vec::new();
            for item in db.iter(&txn)? {
                let (path, _) = item?;
                let path = PathBuf::from(path);
                if !path.exists() {
                    paths_to_delete.push(path);
                }
            }

            for path in paths_to_delete {
                db.delete(&mut txn, &path.to_string_lossy())?;
            }
            txn.commit()?;
            Ok(())
        })
        .await?;

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

async fn summarize_chunks(client: &HuggingFaceClient, chunks: &[String]) -> Result<Vec<String>> {
    let mut chunk_summaries = Vec::new();

    for chunk in chunks {
        let summary = summarize_file(client, chunk).await?;
        chunk_summaries.push(summary);
    }

    Ok(chunk_summaries)
}

async fn summarize_file(client: &HuggingFaceClient, content: &str) -> Result<String> {
    let messages = vec![Message {
        role: "user".to_string(),
        content: format!(
            "You are a code summarization assistant. \
            Provide a brief summary of the given file, \
            focusing on its main functionality and purpose. \
            Be terse and start your response directly with \"Summary: \".\n\
            File:\n{}",
            content
        ),
    }];

    let mut receiver = client
        .stream_completion("tgi".to_string(), messages)
        .await?;

    let mut summary = String::new();
    while let Some(content) = receiver.recv().await {
        summary.push_str(&content);
    }

    Ok(summary)
}

async fn combine_summaries(
    client: &HuggingFaceClient,
    summaries: &[String],
    is_chunk: bool,
) -> Result<String> {
    let combined_content = summaries.join("\n## Summary\n");
    let prompt = if is_chunk {
        concat!(
            "You are a code summarization assistant. ",
            "Combine the given summaries into a single, coherent summary ",
            "that captures the overall functionality and structure of the code. ",
            "Ensure that the final summary is comprehensive and reflects ",
            "the content as if it was summarized from a single, complete file. ",
            "Be terse and start your response with \"Summary: \""
        )
    } else {
        concat!(
            "You are a code summarization assistant. ",
            "Combine the given summaries of different files or directories ",
            "into a single, coherent summary that captures the overall ",
            "structure and functionality of the project or directory. ",
            "Focus on the relationships between different components ",
            "and the high-level architecture. ",
            "Be terse and start your response with \"Summary: \""
        )
    };

    let messages = vec![Message {
        role: "user".to_string(),
        content: format!("{}\n# Summaries\n{}", prompt, combined_content),
    }];

    let mut receiver = client
        .stream_completion("tgi".to_string(), messages)
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

    if args.len() < 2 || args.len() > 4 {
        eprintln!("Usage: {} <project_path> [db_path] [num_workers]", args[0]);
        std::process::exit(1);
    }

    let project_path = Path::new(&args[1]);
    if !project_path.exists() || !project_path.is_dir() {
        eprintln!("Error: The provided project path does not exist or is not a directory.");
        std::process::exit(1);
    }

    let db_path = if args.len() >= 3 {
        PathBuf::from(&args[2])
    } else {
        std::env::current_dir()?.join("project_summaries")
    };

    let num_workers = if args.len() == 4 {
        args[3].parse().unwrap_or(8)
    } else {
        8
    };

    println!("Summarizing project at: {}", project_path.display());
    println!("Using database at: {}", db_path.display());
    println!("Number of workers: {}", num_workers);
    let summary = summarize_project(&db_path, project_path, num_workers).await?;
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
//             .send
