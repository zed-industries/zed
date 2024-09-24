use anyhow::{Context, Result};
use gpui::{AppContext, Model, Task};
use heed::types::{SerdeBincode, Str};
use project::{Entry, UpdatedEntriesSet, Worktree};
use serde::{Deserialize, Serialize};
use smol::channel;
use std::{path::Path, sync::Arc, time::SystemTime};
use tokenizers::Tokenizer;

/// Parameters for the BM25 ranking function.
///
/// BM25 is an extension of TF-IDF that includes two free parameters:
/// - `k1`: Controls term frequency saturation. Higher values give more weight to term frequency.
/// - `b`: Controls document length normalization. A value of 0 means no length normalization,
///        while 1 means full normalization.
///
/// Typical values are k1 ∈ [1.2, 2.0] and b = 0.75.
#[derive(Debug, Clone, Copy)]
pub struct Bm25Params {
    pub k1: f32,
    pub b: f32,
}

impl Default for Bm25Params {
    fn default() -> Self {
        Bm25Params { k1: 1.2, b: 0.75 }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct TfIdfCounts {
    term_frequencies: HashMap<String, u32>,
    document_length: u32,
}

#[derive(Debug, Serialize, Deserialize)]
struct TfIdfMetadata {
    total_document_length: u64,
    document_count: u32,
}

pub struct TfIdfIndex {
    worktree: Model<Worktree>,
    fs: Arc<dyn Fs>,
    db_connection: heed::Env,
    tfidf_db: heed::Database<Str, SerdeBincode<TfIdfCounts>>,
    tokenizer: Tokenizer,
}

impl TfIdfIndex {
    const METADATA_KEY: &'static str = "__tfidf_metadata__";

    pub fn new(
        worktree: Model<Worktree>,
        fs: Arc<dyn Fs>,
        db_connection: heed::Env,
        tfidf_db: heed::Database<Str, SerdeBincode<TfidfCounts>>,
        tokenizer: Tokenizer,
    ) -> Result<Self> {
        let index = Self {
            worktree,
            fs,
            db_connection,
            tfidf_db,
            tokenizer,
        };

        // Initialize metadata if it doesn't exist
        let mut txn = db_connection.write_txn()?;
        if index.get_metadata(&txn)?.is_none() {
            let initial_metadata = TfidfMetadata {
                total_document_length: 0,
                document_count: 0,
            };
            index
                .tfidf_db
                .put(&mut txn, Self::METADATA_KEY, &initial_metadata)?;
            txn.commit()?;
        }

        Ok(index)
    }

    pub fn index_entries_changed_on_disk(
        &self,
        cx: &AppContext,
    ) -> impl Future<Output = Result<()>> {
        let worktree = self.worktree.read(cx).snapshot();
        let worktree_abs_path = worktree.abs_path().clone();
        let (tx, rx) = channel::bounded(512);

        let scan_task = cx.background_executor().spawn(async move {
            for entry in worktree.files(false, 0) {
                tx.send((entry.path.clone(), entry.mtime)).await?;
            }
            Ok(())
        });

        let process_task = self.process_files(rx, worktree_abs_path, cx);

        async move {
            futures::try_join!(scan_task, process_task)?;
            Ok(())
        }
    }

    async fn process_files(
        &self,
        mut rx: channel::Receiver<(Arc<Path>, Option<SystemTime>)>,
        worktree_abs_path: Arc<Path>,
        cx: &AppContext,
    ) -> Result<()> {
        let mut batch = Vec::with_capacity(100);
        while let Some((path, mtime)) = rx.next().await {
            batch.push((path, mtime));
            if batch.len() == 100 {
                self.process_batch(batch, &worktree_abs_path, cx).await?;
                batch = Vec::with_capacity(100);
            }
        }
        if !batch.is_empty() {
            self.process_batch(batch, &worktree_abs_path, cx).await?;
        }
        Ok(())
    }

    async fn process_batch(
        &self,
        batch: Vec<(Arc<Path>, Option<SystemTime>)>,
        worktree_abs_path: &Path,
        cx: &AppContext,
    ) -> Result<()> {
        let futures = batch.into_iter().map(|(path, _mtime)| {
            let abs_path = worktree_abs_path.join(&path);
            async move {
                let counts = self.process_file(&abs_path).await?;
                self.persist_tfidf_counts(&path, counts).await?;
                Ok(())
            }
        });

        futures::future::try_join_all(futures).await?;
        Ok(())
    }

    pub fn index_updated_entries(
        &self,
        updated_entries: UpdatedEntriesSet,
        cx: &AppContext,
    ) -> impl Future<Output = Result<()>> {
        let worktree = self.worktree.read(cx).snapshot();
        let worktree_abs_path = worktree.abs_path().clone();
        let (tx, rx) = channel::bounded(512);

        let scan_task = cx.background_executor().spawn(async move {
            for (path, entry_id, status) in updated_entries.iter() {
                match status {
                    project::PathChange::Added
                    | project::PathChange::Updated
                    | project::PathChange::AddedOrUpdated => {
                        if let Some(entry) = worktree.entry_for_id(*entry_id) {
                            if entry.is_file() {
                                tx.send((entry.path.clone(), entry.mtime)).await?;
                            }
                        }
                    }
                    project::PathChange::Removed => {
                        let mut txn = self.db_connection.write_txn()?;
                        let key = path.to_string_lossy().replace('/', "\0");

                        // Update metadata before deleting the entry
                        if let Some(counts) = self.tfidf_db.get(&txn, &key)? {
                            let mut metadata = self.get_metadata(&txn)?.unwrap();
                            metadata.total_document_length -= counts.document_length as u64;
                            metadata.document_count -= 1;
                            self.tfidf_db.put(&mut txn, Self::METADATA_KEY, &metadata)?;
                        }

                        self.tfidf_db.delete(&mut txn, &key)?;
                        txn.commit()?;
                    }
                    project::PathChange::Loaded => {
                        // Do nothing for loaded entries
                    }
                }
            }
            Ok(())
        });

        let process_task = self.process_files(rx, worktree_abs_path, cx);

        async move {
            futures::try_join!(scan_task, process_task)?;
            Ok(())
        }
    }

    async fn process_file(&self, path: &Path) -> Result<TfIdfCounts> {
        let content = self.fs.read(path).await.context("Failed to read file")?;
        let term_frequencies = self.tokenize_and_count(&content);
        let document_length = term_frequencies.values().sum();

        Ok(TfIdfCounts {
            term_frequencies,
            document_length,
        })
    }

    fn tokenize_and_count(&self, content: &str) -> HashMap<String, u32> {
        let encoding = self.tokenizer.encode(content, true).unwrap();
        let tokens = encoding.get_tokens();

        let mut term_frequencies = HashMap::new();
        for token in tokens {
            *term_frequencies.entry(token.to_string()).or_insert(0) += 1;
        }

        term_frequencies
    }

    async fn persist_tfidf_counts(&self, path: &Path, counts: TfidfCounts) -> Result<()> {
        let mut txn = self.db_connection.write_txn()?;

        let key = path.to_string_lossy().replace('/', "\0");
        let old_counts = self.tfidf_db.get(&txn, &key)?;
        self.tfidf_db.put(&mut txn, &key, &counts)?;

        let mut metadata = self
            .get_metadata(&txn)?
            .expect("TfIdfMetadata should always exist");

        if let Some(old_counts) = old_counts {
            metadata.total_document_length = metadata
                .total_document_length
                .saturating_sub(old_counts.document_length as u64);
            metadata.document_count = metadata.document_count.saturating_sub(1);
        } else {
            metadata.document_count += 1;
        }
        metadata.total_document_length += counts.document_length as u64;

        self.tfidf_db.put(&mut txn, Self::METADATA_KEY, &metadata)?;

        txn.commit()?;
        Ok(())
    }

    fn get_metadata(&self, txn: &RoTxn) -> Result<Option<TfidfMetadata>> {
        self.tfidf_db.get(txn, Self::METADATA_KEY)
    }

    pub async fn calculate_idf(&self) -> Result<HashMap<String, f32>> {
        let mut document_frequency: HashMap<String, u32> = HashMap::new();

        let txn = self.db_connection.read_txn()?;
        let metadata = self
            .get_metadata(&txn)?
            .ok_or_else(|| anyhow::anyhow!("Metadata not found"))?;
        let total_documents = metadata.document_count;

        let mut iter = self.tfidf_db.iter(&txn)?;
        let mut counted_documents = 0;
        while let Some(entry) = iter.next() {
            let (_, counts) = entry?;
            counted_documents += 1;
            for term in counts.term_frequencies.keys() {
                *document_frequency.entry(term.clone()).or_insert(0) += 1;
            }
        }

        if counted_documents != total_documents {
            log::warn!("Inconsistency detected: Metadata document count ({}) doesn't match actual document count ({})", total_documents, counted_documents);
        }

        let idf: HashMap<String, f32> = document_frequency
            .into_iter()
            .map(|(term, df)| {
                let idf = (total_documents as f32 / df as f32).ln() + 1.0;
                (term, idf)
            })
            .collect();

        Ok(idf)
    }

    /// Performs a BM25 search on the indexed documents.
    ///
    /// BM25 is a bag-of-words retrieval function that ranks a set of documents based on the query terms
    /// appearing in each document. It extends the TF-IDF model by normalizing term frequency saturation
    /// and document length normalization.
    ///
    /// The BM25 score for a document D and query Q is:
    ///
    /// score(D,Q) = sum(IDF(qi) * (f(qi,D) * (k1 + 1)) / (f(qi,D) + k1 * (1 - b + b * |D| / avgdl)))
    ///
    /// Where:
    /// - IDF(qi) is the Inverse Document Frequency of query term qi
    /// - f(qi,D) is the term frequency of qi in document D
    /// - |D| is the length of document D
    /// - avgdl is the average document length in the collection
    /// - k1 and b are free parameters (usually, k1 ∈ [1.2, 2.0] and b = 0.75)
    pub async fn query(&self, query: &str, params: &Bm25Params) -> Result<Vec<(f32, Arc<Path>)>> {
        let idf = self.calculate_idf().await?;
        let query_terms = self.tokenize_and_count(query);
        let k1 = params.k1;
        let b = params.b;
        let mut scores = Vec::new();

        let txn = self.db_connection.read_txn()?;

        // Fetch metadata and calculate average document length
        let metadata = self
            .get_metadata(&txn)?
            .ok_or_else(|| anyhow::anyhow!("Metadata not found"))?;
        let avg_doc_len = metadata.total_document_length as f32 / metadata.document_count as f32;

        let mut iter = self.tfidf_db.iter(&txn)?;
        while let Some(entry) = iter.next() {
            let (path, counts) = entry?;
            let mut score = 0.0;

            for (term, query_tf) in &query_terms {
                if let Some(&doc_tf) = counts.term_frequencies.get(term) {
                    if let Some(&term_idf) = idf.get(term) {
                        let tf = doc_tf as f32;
                        let doc_len = counts.document_length as f32;

                        let numerator = tf * (k1 + 1.0);
                        let denominator = tf + k1 * (1.0 - b + b * doc_len / avg_doc_len);
                        score += term_idf * numerator / denominator * *query_tf as f32;
                    }
                }
            }

            if score > 0.0 {
                scores.push((score, Arc::new(Path::new(path))));
            }
        }

        scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));
        Ok(scores)
    }
}
