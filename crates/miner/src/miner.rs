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
    sync::{atomic::AtomicUsize, Arc},
    time::SystemTime,
};
use tokenizers::{tokenizer::Tokenizer, FromPretrainedParameters};
use tokio::sync::{mpsc, Mutex};

const CHUNK_SIZE: usize = 5000;
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

    /// Export the database contents to stdout
    #[arg(long)]
    export: bool,
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

    if cli.export {
        miner.export_database().await?;
        return Ok(());
    }

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
    _multi_progress: Arc<MultiProgress>,
    overall_progress: ProgressBar,
    file_progress: ProgressBar,
    chunk_progress: ProgressBar,
    rust_symbol_progress: ProgressBar,
    summaries: Arc<Mutex<BTreeMap<PathBuf, String>>>,
    paths_loaded_from_cache: Arc<Mutex<BTreeMap<PathBuf, bool>>>,
    file_progress_map: Arc<Mutex<HashMap<PathBuf, FileProgress>>>,
    outstanding_chunks: Arc<AtomicUsize>,
    outstanding_symbols: Arc<AtomicUsize>,
    progress_sender: mpsc::UnboundedSender<()>,
    total_chunks: Arc<AtomicUsize>,
    total_symbols: Arc<AtomicUsize>,
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

        let file_progress = multi_progress.add(ProgressBar::new(0));
        let chunk_progress = multi_progress.add(ProgressBar::new(0));
        let rust_symbol_progress = multi_progress.add(ProgressBar::new(0));

        for pb in [&file_progress, &chunk_progress, &rust_symbol_progress] {
            pb.set_style(
                ProgressStyle::default_bar()
                    .template("[{elapsed_precise}] {bar:40.cyan/blue} {pos}/{len} {msg}")
                    .unwrap()
                    .progress_chars("##-"),
            );
        }

        let summaries = Arc::new(Mutex::new(BTreeMap::new()));
        let paths_loaded_from_cache = Arc::new(Mutex::new(BTreeMap::new()));

        let file_progress_map = Arc::new(Mutex::new(HashMap::new()));
        let outstanding_chunks = Arc::new(AtomicUsize::new(0));
        let outstanding_symbols = Arc::new(AtomicUsize::new(0));
        let total_chunks = Arc::new(AtomicUsize::new(0));
        let total_symbols = Arc::new(AtomicUsize::new(0));

        let (progress_sender, mut progress_receiver) = mpsc::unbounded_channel();

        let miner = Arc::new(Self {
            root,
            num_workers,
            database,
            tokenizer,
            client,
            queue,
            _multi_progress: Arc::clone(&multi_progress),
            overall_progress,
            file_progress,
            chunk_progress,
            rust_symbol_progress,
            summaries,
            paths_loaded_from_cache,
            file_progress_map,
            outstanding_chunks,
            outstanding_symbols,
            progress_sender,
            total_chunks,
            total_symbols,
        });

        let miner_clone = Arc::clone(&miner);
        tokio::spawn(async move {
            while progress_receiver.recv().await.is_some() {
                miner_clone.do_update_progress().await;
            }
        });

        Ok(miner)
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
                    self.queue
                        .lock()
                        .await
                        .push_back(Entry::Directory(path.clone()));
                } else {
                    self.queue.lock().await.push_back(Entry::File(path.clone()));
                }
                self.file_progress_map.lock().await.insert(
                    path,
                    FileProgress {
                        outstanding_chunks: 0,
                        outstanding_symbols: 0,
                        is_complete: false,
                    },
                );
            }
        }

        self.update_progress();

        let workers: Vec<_> = (0..self.num_workers)
            .map(|_| {
                let this = self.clone();
                tokio::spawn(async move { this.worker().await })
            })
            .collect();

        for worker in workers {
            worker.await??;
        }

        self.update_progress();

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
                Ok(())
            })
            .await?;

        self.file_progress
            .finish_with_message("File processing complete");
        self.chunk_progress
            .finish_with_message("Chunk processing complete");
        self.rust_symbol_progress
            .finish_with_message("Rust symbol processing complete");
        self.overall_progress
            .finish_with_message("Project summarization finished");
        Ok(())
    }

    async fn worker(self: &Arc<Self>) -> Result<()> {
        loop {
            let entry = {
                let mut queue_lock = self.queue.lock().await;
                queue_lock.pop_front()
            };

            match entry {
                Some(Entry::File(path)) => {
                    let content = tokio::fs::read_to_string(&path).await.unwrap_or_default();
                    self.scan_file(path, content).await?;
                }
                Some(Entry::Directory(path)) => {
                    self.scan_directory(path).await?;
                }
                Some(Entry::Chunk(path, content, index)) => {
                    self.process_chunk(path, content, index).await?;
                }
                Some(Entry::RustSymbol(path, name, content)) => {
                    self.process_rust_symbol(path, name, content).await?;
                }
                None => break,
            }
            self.update_progress();
        }

        Ok(())
    }

    async fn scan_file(&self, path: PathBuf, content: String) -> Result<()> {
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
                self.summaries
                    .lock()
                    .await
                    .insert(path.clone(), cached.summary);
                self.file_progress_map
                    .lock()
                    .await
                    .get_mut(&path)
                    .map(|progress| {
                        progress.is_complete = true;
                    });
                self.update_progress();
                return Ok(());
            }
        }

        self.file_progress
            .set_message(format!("Summarizing {}", path.display()));

        if path.extension().map_or(false, |ext| ext == "rs") {
            match self
                .parse_and_enqueue_rust_symbols(path.clone(), &content)
                .await
            {
                Ok(symbol_count) => {
                    self.file_progress_map
                        .lock()
                        .await
                        .get_mut(&path)
                        .map(|progress| {
                            progress.outstanding_symbols = symbol_count;
                        });
                    self.update_progress();
                    return Ok(());
                }
                Err(e) => {
                    eprintln!(
                        "Error parsing Rust symbols for {}: {}\nProcessing as text instead",
                        path.display(),
                        e
                    );
                }
            }
        }

        if self.count_tokens(&content) > CHUNK_SIZE {
            let chunk_count = self.split_and_enqueue_chunks(path.clone(), content).await?;
            self.file_progress_map
                .lock()
                .await
                .get_mut(&path)
                .map(|progress| {
                    progress.outstanding_chunks = chunk_count;
                });
        } else {
            let summary = self.summarize_file(&path, &content).await?;
            let cached_summary = CachedSummary {
                summary: summary.clone(),
                mtime,
            };
            self.database
                .transact(move |db, mut txn| {
                    db.put(&mut txn, &key, &cached_summary)?;
                    Ok(())
                })
                .await?;
            self.summaries.lock().await.insert(path.clone(), summary);
            self.file_progress_map
                .lock()
                .await
                .get_mut(&path)
                .map(|progress| {
                    progress.is_complete = true;
                });
        }

        self.update_progress();
        Ok(())
    }

    async fn parse_and_enqueue_rust_symbols(&self, path: PathBuf, content: &str) -> Result<usize> {
        let mut parser = tree_sitter::Parser::new();
        parser.set_language(&tree_sitter_rust::language())?;
        let tree = parser
            .parse(content, None)
            .ok_or_else(|| anyhow!("Failed to parse content"))?;
        let root_node = tree.root_node();

        let export_query = tree_sitter::Query::new(
            &tree_sitter_rust::language(),
            include_str!("./rust_exports.scm"),
        )?;

        let mut export_cursor = tree_sitter::QueryCursor::new();
        let mut symbols = Vec::new();
        for m in export_cursor.matches(&export_query, root_node, content.as_bytes()) {
            if let Some(capture) = m.captures.first() {
                let symbol_name = content[capture.node.byte_range()].to_string();
                let symbol_content =
                    content[capture.node.start_byte()..capture.node.end_byte()].to_string();
                symbols.push((symbol_name, symbol_content));
            }
        }
        let mut queue = self.queue.lock().await;
        let symbol_count = symbols.len();
        for (symbol_name, symbol_content) in symbols {
            queue.push_back(Entry::RustSymbol(path.clone(), symbol_name, symbol_content));
        }

        self.outstanding_symbols
            .fetch_add(symbol_count, std::sync::atomic::Ordering::SeqCst);
        self.total_symbols
            .fetch_add(symbol_count, std::sync::atomic::Ordering::SeqCst);
        self.update_progress();
        Ok(symbol_count)
    }

    async fn process_chunk(&self, path: PathBuf, content: String, index: usize) -> Result<()> {
        let summary = self.summarize_file(&path, &content).await?;
        let key = format!("{}_{}", path.to_string_lossy(), index);
        let mtime = tokio::fs::metadata(&path).await?.modified()?;
        let cached_summary = CachedSummary {
            summary: summary.clone(),
            mtime,
        };
        self.database
            .transact(move |db, mut txn| {
                db.put(&mut txn, &key, &cached_summary)?;
                Ok(())
            })
            .await?;
        self.summaries
            .lock()
            .await
            .entry(path.clone())
            .or_insert_with(String::new)
            .push_str(&summary);

        let mut file_progress = self.file_progress_map.lock().await;
        if let Some(progress) = file_progress.get_mut(&path) {
            progress.outstanding_chunks -= 1;

            if progress.outstanding_chunks == 0 && progress.outstanding_symbols == 0 {
                progress.is_complete = true;
            }
        }
        drop(file_progress);
        self.update_progress();

        Ok(())
    }

    async fn split_and_enqueue_chunks(&self, path: PathBuf, content: String) -> Result<usize> {
        let chunks = self.split_into_chunks(&content);
        let chunk_count = chunks.len();
        for (index, chunk) in chunks.into_iter().enumerate() {
            self.queue
                .lock()
                .await
                .push_back(Entry::Chunk(path.clone(), chunk, index));
        }
        self.outstanding_chunks
            .fetch_add(chunk_count, std::sync::atomic::Ordering::SeqCst);
        self.total_chunks
            .fetch_add(chunk_count, std::sync::atomic::Ordering::SeqCst);
        self.update_progress();
        Ok(chunk_count)
    }

    async fn process_rust_symbol(
        &self,
        path: PathBuf,
        name: String,
        content: String,
    ) -> Result<()> {
        let summary = self.summarize_rust_symbol(&name, &content).await?;
        self.summaries.lock().await.insert(path.clone(), summary);
        let previous_outstanding = self
            .outstanding_symbols
            .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        let mut file_progress = self.file_progress_map.lock().await;
        if let Some(progress) = file_progress.get_mut(&path) {
            progress.outstanding_symbols -= 1;
            if progress.outstanding_symbols == 0 && progress.outstanding_chunks == 0 {
                progress.is_complete = true;
            }
        }
        drop(file_progress);
        self.update_progress();
        Ok(())
    }

    async fn summarize_rust_symbol(&self, name: &str, content: &str) -> Result<String> {
        let messages = vec![Message {
            role: "user".to_string(),
            content: format!(
                "You are a code summarization assistant. \
                Provide a brief summary of the given Rust symbol, \
                focusing on its main functionality and purpose. \
                Be terse and start your response directly with \"Summary: \".\n\
                Symbol name: {}\n\
                Symbol content:\n{}",
                name, content
            ),
        }];

        let mut receiver = self.client.stream_completion(messages).await?;

        let mut summary = String::new();
        while let Some(content) = receiver.recv().await {
            summary.push_str(&content);
        }

        Ok(summary)
    }

    fn count_tokens(&self, content: &str) -> usize {
        self.tokenizer
            .encode(content, false)
            .unwrap()
            .get_ids()
            .len()
    }

    async fn summarize_file(&self, path: &Path, content: &str) -> Result<String> {
        let messages = vec![Message {
            role: "user".to_string(),
            content: format!(
                "You are a code summarization assistant. \
                Provide a brief summary of the given file, \
                focusing on its main functionality and purpose. \
                Be terse and start your response directly with \"Summary: \".\n\
                File path: {}\n\
                File content:\n{}",
                path.display(),
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

    async fn scan_directory(&self, path: PathBuf) -> Result<()> {
        let dir_walker = ignore::WalkBuilder::new(&path)
            .hidden(true)
            .ignore(true)
            .max_depth(Some(1))
            .build();

        for entry in dir_walker {
            if let Ok(entry) = entry {
                if entry.path() != path {
                    let entry_path = entry.path().to_path_buf();
                    if entry.file_type().map_or(false, |ft| ft.is_dir()) {
                        self.queue
                            .lock()
                            .await
                            .push_back(Entry::Directory(entry_path));
                    } else {
                        self.queue.lock().await.push_back(Entry::File(entry_path));
                    }
                }
            }
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

    fn update_progress(&self) {
        let _ = self.progress_sender.send(());
    }

    async fn do_update_progress(&self) {
        let (completed_files, total_files) = {
            let map = self.file_progress_map.lock().await;
            let total = map.len();
            let completed = map.values().filter(|v| v.is_complete).count();
            (completed, total)
        };
        let outstanding_chunks = self
            .outstanding_chunks
            .load(std::sync::atomic::Ordering::SeqCst);
        let outstanding_symbols = self
            .outstanding_symbols
            .load(std::sync::atomic::Ordering::SeqCst);
        let total_chunks = self.total_chunks.load(std::sync::atomic::Ordering::SeqCst);
        let total_symbols = self.total_symbols.load(std::sync::atomic::Ordering::SeqCst);

        let completed_chunks = total_chunks.saturating_sub(outstanding_chunks);
        let completed_symbols = total_symbols.saturating_sub(outstanding_symbols);

        self.file_progress.set_position(completed_files as u64);
        self.file_progress.set_length(total_files as u64);
        self.file_progress.set_message(format!("Files processed"));
        self.chunk_progress.set_position(completed_chunks as u64);
        self.chunk_progress.set_length(total_chunks as u64);
        self.chunk_progress.set_message(format!("Chunks processed"));
        self.rust_symbol_progress
            .set_position(completed_symbols as u64);
        self.rust_symbol_progress.set_length(total_symbols as u64);
        self.rust_symbol_progress
            .set_message(format!("Rust symbols processed"));

        // Update overall progress
        let total_work = total_files + total_chunks + total_symbols;
        let completed_work = completed_files + completed_chunks + completed_symbols;
        self.overall_progress.set_position(completed_work as u64);
        self.overall_progress.set_length(total_work as u64);
        self.overall_progress.set_message(format!(
            "Overall progress: {:.1}%",
            (completed_work as f64 / total_work.max(1) as f64) * 100.0
        ));
    }

    pub async fn export_database(&self) -> Result<()> {
        self.database
            .transact(|db, txn| {
                for item in db.iter(&txn)? {
                    let (key, value) = item?;
                    let entry = serde_json::json!({
                        "path": key,
                        "summary": value.summary,
                        "mtime": value.mtime.duration_since(SystemTime::UNIX_EPOCH)?.as_secs()
                    });
                    println!("{}", serde_json::to_string(&entry)?);
                }
                Ok(())
            })
            .await
    }
}

#[derive(Debug)]
enum Entry {
    File(PathBuf),
    Directory(PathBuf),
    Chunk(PathBuf, String, usize),
    RustSymbol(PathBuf, String, String),
}

#[derive(Debug, Serialize, Deserialize)]
struct CachedSummary {
    summary: String,
    mtime: SystemTime,
}

struct FileProgress {
    outstanding_chunks: usize,
    outstanding_symbols: usize,
    is_complete: bool,
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
        F: FnOnce(&HeedDatabase<Str, SerdeJson<CachedSummary>>, &mut RwTxn) -> Result<T>
            + Send
            + 'static,
        T: 'static + Send,
    {
        let (tx, rx) = tokio::sync::oneshot::channel();
        self.tx
            .send(Box::new(move |db, mut txn| {
                let result = f(db, &mut txn);
                if result.is_ok() {
                    if let Err(e) = txn.commit() {
                        let _ = tx.send(Err(anyhow::Error::from(e)));
                        return;
                    }
                }
                let _ = tx.send(result);
            }))
            .await
            .map_err(|_| anyhow!("database closed"))?;
        Ok(rx.await.map_err(|_| anyhow!("transaction failed"))??)
    }
}
