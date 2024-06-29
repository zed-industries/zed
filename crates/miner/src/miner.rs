mod huggingface;
mod ollama;

use anyhow::{anyhow, Result};
use clap::Parser;
use heed::{
    types::{SerdeJson, Str},
    Database as HeedDatabase, EnvOpenOptions, RwTxn,
};
use huggingface::HuggingFaceClient;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap, VecDeque},
    path::{Path, PathBuf},
    sync::Arc,
    time::SystemTime,
};
use tokenizers::{tokenizer::Tokenizer, FromPretrainedParameters};
use tokio::sync::{mpsc, Mutex};

const CHUNK_SIZE: usize = 5000;
const OVERLAP: usize = 2_000;

const HUGGINGFACE_ENDPOINT_URL: &str =
    "https://eezviumpj7crpq2t.us-east-1.aws.endpoints.huggingface.cloud";

#[derive(Parser)]
#[command(name = "Project Summarizer")]
#[command(author = "Your Name")]
#[command(version = "1.0")]
#[command(about = "Summarizes a project directory", long_about = None)]
struct Cli {
    /// The path to the project directory
    project_path: PathBuf,

    /// The path to the database
    #[arg(short = 'd', long = "db-path")]
    db_path: Option<PathBuf>,

    /// Number of worker threads
    #[arg(short = 'w', long = "workers", default_value = "8")]
    num_workers: usize,

    /// Path to read summaries from
    #[arg(long)]
    read: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let project_path = &cli.project_path;
    if !project_path.exists() || !project_path.is_dir() {
        eprintln!("Error: The provided project path does not exist or is not a directory.");
        std::process::exit(1);
    }

    let db_path = cli
        .db_path
        .unwrap_or_else(|| std::env::current_dir().unwrap().join("project_summaries"));

    println!("Summarizing project at: {}", project_path.display());
    println!("Using database at: {}", db_path.display());
    println!("Number of workers: {}", cli.num_workers);

    let miner = Miner::new(db_path, project_path.to_path_buf(), cli.num_workers).await?;
    miner.summarize_project().await?;

    println!("Finished summarization");

    if let Some(read_path) = cli.read {
        let full_path = project_path.join(&read_path);
        if let Some(summary) = miner.summary_for_path(&full_path).await? {
            println!("<path>{}</path>", full_path.to_string_lossy());
            println!("<summary>{}</summary>", summary);
            println!();
        }
    }

    Ok(())
}

pub struct Miner {
    root: PathBuf,
    num_workers: usize,
    database: Database,
    tokenizer: Tokenizer,
    client: Arc<HuggingFaceClient>,
    queue: Arc<Mutex<VecDeque<Entry>>>,
    multi_progress: Arc<MultiProgress>,
    overall_progress: ProgressBar,
    progress_bar: ProgressBar,
    summaries: Arc<Mutex<BTreeMap<PathBuf, String>>>,
    paths_loaded_from_cache: Arc<Mutex<BTreeMap<PathBuf, bool>>>,
}

impl Miner {
    pub async fn new(db_path: PathBuf, root: PathBuf, num_workers: usize) -> Result<Arc<Self>> {
        let database = Database::new(&db_path, &root).await?;

        let tokenizer = Tokenizer::from_pretrained(
            "Qwen/Qwen2-7B-Instruct",
            Some(FromPretrainedParameters {
                revision: "main".into(),
                user_agent: HashMap::default(),
                auth_token: Some(
                    std::env::var("HUGGINGFACE_API_KEY").expect("HUGGINGFACE_API_KEY not set"),
                ),
            }),
        )
        .unwrap();

        let client = Arc::new(HuggingFaceClient::new(
            HUGGINGFACE_ENDPOINT_URL.to_string(),
            std::env::var("HUGGINGFACE_API_KEY").expect("HUGGINGFACE_API_KEY not set"),
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

        let progress_bar = multi_progress.add(ProgressBar::new(0));
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                .unwrap()
                .progress_chars("##-"),
        );

        let summaries = Arc::new(Mutex::new(BTreeMap::new()));
        let paths_loaded_from_cache = Arc::new(Mutex::new(BTreeMap::new()));

        Ok(Arc::new(Self {
            root,
            num_workers,
            database,
            tokenizer,
            client,
            queue,
            multi_progress,
            overall_progress,
            progress_bar,
            summaries,
            paths_loaded_from_cache,
        }))
    }

    pub async fn summarize_project(self: &Arc<Self>) -> Result<()> {
        // Populate the queue with files and directories
        let mut walker = ignore::WalkBuilder::new(&self.root)
            .hidden(true)
            .ignore(true)
            .build();
        while let Some(entry) = walker.next() {
            if let Ok(entry) = entry {
                let path = entry.path().to_owned();
                if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                    self.queue.lock().await.push_back(Entry::Directory(path));
                } else {
                    self.queue.lock().await.push_back(Entry::File(path));
                }
            }
        }

        let total_entries = self.queue.lock().await.len();
        self.progress_bar.set_length(total_entries as u64);

        let workers: Vec<_> = (0..self.num_workers)
            .map(|_| {
                let this = self.clone();
                tokio::spawn(async move { this.worker().await })
            })
            .collect();

        for worker in workers {
            worker.await??;
        }

        // Remove deleted entries from the database
        self.database
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

        self.progress_bar
            .finish_with_message("Summarization complete");
        self.overall_progress
            .finish_with_message("Project summarization finished");
        Ok(())
    }

    async fn worker(self: &Arc<Self>) -> Result<()> {
        loop {
            let mut queue_lock = self.queue.lock().await;
            let Some(entry) = queue_lock.pop_front() else {
                break;
            };

            match entry {
                Entry::File(path) => {
                    drop(queue_lock);
                    self.scan_file(path).await?;
                }
                Entry::Directory(path) => {
                    self.scan_directory(path, &mut queue_lock).await?;
                }
            }
        }

        Ok(())
    }

    async fn scan_file(&self, path: PathBuf) -> Result<()> {
        let summary = async {
            let mtime = tokio::fs::metadata(&path).await?.modified()?;
            let key = path.to_string_lossy().to_string();

            let cached = self
                .database
                .transact({
                    let key = key.clone();
                    move |db, txn| Ok(db.get(&txn, &key)?)
                })
                .await?;
            if let Some(cached) = cached {
                if cached.mtime == mtime {
                    self.paths_loaded_from_cache
                        .lock()
                        .await
                        .insert(path.clone(), true);
                    return Ok(cached.summary);
                }
            }

            self.progress_bar
                .set_message(format!("Summarizing {}", path.display()));

            let content = tokio::fs::read_to_string(&path)
                .await
                .unwrap_or_else(|_| "binary file".into());

            let summary = self.summarize_file(&path, &content).await?;

            let cached_summary = CachedSummary {
                summary: summary.clone(),
                mtime,
            };
            self.database
                .transact(move |db, mut txn| {
                    db.put(&mut txn, &key, &cached_summary)?;
                    txn.commit()?;
                    Ok(())
                })
                .await?;

            anyhow::Ok(summary)
        };

        let summary = summary
            .await
            .unwrap_or_else(|error| format!("path could not be summarized: {error:?}"));
        self.summaries.lock().await.insert(path, summary);
        self.progress_bar.inc(1);

        Ok(())
    }

    fn count_tokens(&self, content: &str) -> usize {
        self.tokenizer
            .encode(content, false)
            .unwrap()
            .get_ids()
            .len()
    }

    async fn summarize_file(&self, path: &Path, content: &str) -> Result<String> {
        if path.extension().map_or(false, |ext| ext == "rs") {
            self.summarize_rust_file(content)
        } else {
            let token_count = self.count_tokens(content);

            println!("Token count for file {}: {}", path.display(), token_count);

            if token_count > CHUNK_SIZE {
                let chunks = self.split_into_chunks(content);
                println!("Chunking file: {}", path.display());
                println!("Tokenized chunk lengths:");
                for (i, chunk) in chunks.iter().enumerate() {
                    let token_count = self.count_tokens(chunk);
                    println!("  Chunk {}: {} tokens", i + 1, token_count);
                }
                let chunk_summaries = Box::pin(self.summarize_chunks(path, &chunks)).await?;
                println!("Chunk summaries: {:?}", chunk_summaries);

                let combined_content = chunk_summaries.join("\n## Summary\n");
                let messages = vec![
                    Message {
                        role: "system".to_string(),
                        content: concat!(
                            "You are a code summarization assistant. ",
                            "Combine the given summaries into a single, coherent summary ",
                            "that captures the overall functionality and structure of the code. ",
                            "Ensure that the final summary is comprehensive and reflects ",
                            "the content as if it was summarized from a single, complete file. ",
                            "Be terse and start your response with \"Summary: \""
                        )
                        .to_string(),
                    },
                    Message {
                        role: "user".to_string(),
                        content: format!("# Summaries\n{}", combined_content),
                    },
                ];

                let mut receiver = self.client.stream_completion(messages).await?;

                let mut combined_summary = String::new();
                while let Some(content) = receiver.recv().await {
                    combined_summary.push_str(&content);
                }

                Ok(combined_summary)
            } else {
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

                let mut receiver = self.client.stream_completion(messages).await?;

                let mut summary = String::new();
                while let Some(content) = receiver.recv().await {
                    summary.push_str(&content);
                }

                Ok(summary)
            }
        }
    }

    fn summarize_rust_file(&self, content: &str) -> Result<String> {
        let mut summary = String::new();
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_rust::language()).unwrap();
        let tree = parser.parse(content, None).unwrap();
        let root_node = tree.root_node();

        let export_query = tree_sitter::Query::new(
            &tree_sitter_rust::language(),
            include_str!("./rust_exports.scm"),
        )
        .unwrap();

        let mut export_cursor = tree_sitter::QueryCursor::new();
        let mut exports = Vec::new();
        for m in export_cursor.matches(&export_query, root_node, content.as_bytes()) {
            let mut current_level = 0;
            let mut current_export = String::new();
            for c in m.captures {
                let export = content[c.node.byte_range()].to_string();
                let indent = "  ".repeat(current_level);
                if current_level == 0 {
                    current_export = format!("{}{}", indent, export);
                } else {
                    current_export.push_str(&format!("\n{}{}", indent, export));
                }
                current_level += 1;
            }
            exports.push(current_export);
        }

        let import_query = tree_sitter::Query::new(
            &tree_sitter_rust::language(),
            include_str!("./rust_imports.scm"),
        )
        .unwrap();
        let mut import_cursor = tree_sitter::QueryCursor::new();
        let imports: Vec<_> = import_cursor
            .matches(&import_query, root_node, content.as_bytes())
            .flat_map(|m| m.captures)
            .map(|c| content[c.node.byte_range()].to_string())
            .collect();

        summary.push_str("Summary: Rust file containing ");
        if !exports.is_empty() {
            summary.push_str(&format!("{} exports", exports.len()));
            if !imports.is_empty() {
                summary.push_str(" and ");
            }
        }
        if !imports.is_empty() {
            summary.push_str(&format!("{} imports", imports.len()));
        }
        summary.push('.');

        if !exports.is_empty() {
            summary.push_str("\nExports:\n");
            summary.push_str(&exports.join("\n"));
        }
        if !imports.is_empty() {
            summary.push_str("\nImports: ");
            summary.push_str(&imports.join(", "));
        }

        Ok(summary)
    }

    fn split_into_chunks(&self, content: &str) -> Vec<String> {
        let mut chunks = Vec::new();
        let lines: Vec<&str> = content.lines().collect();
        let mut current_chunk = String::new();
        let mut current_chunk_token_count = 0;

        for line in lines {
            let encoded = self.tokenizer.encode(line, false).unwrap();
            let line_tokens = encoded.get_ids();
            let line_token_count = line_tokens.len();

            if current_chunk_token_count + line_token_count > CHUNK_SIZE {
                // Flush the current chunk
                chunks.push(current_chunk.clone());
                current_chunk.clear();
                current_chunk_token_count = 0;
            }

            if line_token_count > CHUNK_SIZE {
                // Truncate the line and append it
                for token in encoded.get_tokens().into_iter().take(CHUNK_SIZE) {
                    current_chunk.push_str(token);
                }
                chunks.push(current_chunk.clone());
                current_chunk.clear();
                current_chunk_token_count = 0;
            } else {
                // Add the line to the current chunk
                current_chunk.push_str(line);
                current_chunk.push('\n');
                current_chunk_token_count += line_token_count;
            }
        }

        if !current_chunk.is_empty() {
            chunks.push(current_chunk);
        }

        chunks
    }

    async fn summarize_chunks(&self, path: &Path, chunks: &[String]) -> Result<Vec<String>> {
        let mut chunk_summaries = Vec::new();

        for chunk in chunks {
            let token_count = self.count_tokens(chunk);
            println!("Tokenized length of chunk: {}", token_count);
            let summary = self.summarize_file(path, chunk).await?;
            chunk_summaries.push(summary);
        }

        Ok(chunk_summaries)
    }

    async fn scan_directory(
        &self,
        path: PathBuf,
        queue_lock: &mut tokio::sync::MutexGuard<'_, VecDeque<Entry>>,
    ) -> Result<()> {
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
                    if let Some(summary) = self.summaries.lock().await.get(entry.path()) {
                        dir_summaries.push(summary.clone());
                        if !self
                            .paths_loaded_from_cache
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
            let combined_summary = async {
                let key = path.to_string_lossy().to_string();
                let mtime = tokio::fs::metadata(&path).await?.modified()?;

                if all_children_from_cache {
                    if let Some(cached) = self
                        .database
                        .transact({
                            let key = key.clone();
                            move |db, txn| Ok(db.get(&txn, &key)?)
                        })
                        .await?
                    {
                        self.paths_loaded_from_cache
                            .lock()
                            .await
                            .insert(path.clone(), true);
                        return Ok(cached.summary);
                    }
                }

                self.progress_bar
                    .set_message(format!("Summarizing {}", path.display()));

                let combined_content = dir_summaries.join("\n## Summary\n");
                let messages = vec![
                    Message {
                        role: "system".to_string(),
                        content: concat!(
                            "You are a code summarization assistant. ",
                            "Combine the given summaries of different files or directories ",
                            "into a single, coherent summary that captures the overall ",
                            "structure and functionality of the project or directory. ",
                            "Focus on the relationships between different components ",
                            "and the high-level architecture. ",
                            "Be terse and start your response with \"Summary: \""
                        )
                        .to_string(),
                    },
                    Message {
                        role: "user".to_string(),
                        content: format!("# Summaries\n{}", combined_content),
                    },
                ];

                let mut receiver = self.client.stream_completion(messages).await?;

                let mut combined_summary = String::new();
                while let Some(content) = receiver.recv().await {
                    combined_summary.push_str(&content);
                }

                let cached_summary = CachedSummary {
                    summary: combined_summary.clone(),
                    mtime,
                };
                self.database
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

            self.summaries.lock().await.insert(path, combined_summary);
            self.progress_bar.inc(1);
        } else {
            queue_lock.push_back(Entry::Directory(path));
        }

        Ok(())
    }

    async fn summary_for_path(&self, path: &Path) -> Result<Option<String>> {
        let key = path.to_string_lossy().to_string();
        let cached_summary = self
            .database
            .transact(move |db, txn| Ok(db.get(&txn, &key)?))
            .await?;

        if let Some(cached) = cached_summary {
            return Ok(Some(cached.summary));
        }

        if let Some(summary) = self.summaries.lock().await.get(path) {
            return Ok(Some(summary.clone()));
        }

        Ok(None)
    }
}

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

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
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
