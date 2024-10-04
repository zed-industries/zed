use rust_stemmers::{Algorithm, Stemmer};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
    sync::Arc,
};

#[derive(Clone)]
pub struct SimpleTokenizer {
    stemmer: Arc<Stemmer>,
}

impl SimpleTokenizer {
    pub fn new() -> Self {
        Self {
            stemmer: Arc::new(Stemmer::create(Algorithm::English)), // We can make this configurable later
        }
    }

    pub fn tokenize_and_stem(&self, text: &str) -> Vec<Arc<str>> {
        text.split_whitespace()
            .map(|word| {
                let stemmed = self.stemmer.stem(word).to_string();
                Arc::from(stemmed)
            })
            .collect()
    }
}

pub struct Bm25Parameters {
    pub k1: f32,
    pub b: f32,
}

impl Default for Bm25Parameters {
    fn default() -> Self {
        Self { k1: 1.2, b: 0.75 }
    }
}
pub trait Bm25Scorer {
    fn total_chunks(&self) -> u64;
    fn avg_chunk_length(&self) -> f32;
    fn term_frequency(&self, term: &Arc<str>, chunk_id: u64) -> Option<u32>;
    fn chunk_length(&self, chunk_id: u64) -> Option<u32>;
    fn document_frequency(&self, term: &Arc<str>) -> u64;

    fn calculate_bm25_score(
        &self,
        query_terms: &HashMap<Arc<str>, f32>,
        chunk_id: u64,
        k1: f32,
        b: f32,
    ) -> Option<f32> {
        let avg_dl = self.avg_chunk_length();
        let chunk_length = self.chunk_length(chunk_id)? as f32;

        Some(
            query_terms
                .iter()
                .filter_map(|(term, &query_tf)| {
                    let tf = self.term_frequency(term, chunk_id)? as f32;
                    let df = self.document_frequency(term) as f32;
                    let idf = ((self.total_chunks() as f32 - df + 0.5) / (df + 0.5)).ln();
                    let numerator = tf * (k1 + 1.0);
                    let denominator = tf + k1 * (1.0 - b + b * chunk_length / avg_dl);

                    Some(query_tf * idf * (numerator / denominator))
                })
                .sum(),
        )
    }
}

#[derive(Debug)]
pub struct TermStats {
    frequency: u32,
    chunk_ids: HashSet<u64>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChunkStats {
    length: u32,
    terms: HashMap<Arc<str>, u32>,
}

impl ChunkStats {
    pub fn from_text(text: &str, tokenizer: &SimpleTokenizer) -> Self {
        let tokens = tokenizer.tokenize_and_stem(text);
        let mut terms = HashMap::new();
        let length = tokens.len() as u32;

        for token in tokens {
            *terms.entry(token).or_insert(0) += 1;
        }

        ChunkStats { length, terms }
    }
}

/// Represents the term frequency statistics for a single worktree.
///
/// This struct contains information about chunks, term statistics,
/// and the total length of all chunks in the worktree.
#[derive(Debug)]
pub struct WorktreeTermStats {
    /// A map of chunk IDs to their corresponding statistics.
    chunks: HashMap<u64, ChunkStats>,
    /// A map of terms to their statistics across all chunks in this worktree.
    term_stats: HashMap<Arc<str>, TermStats>,
    /// The total length of all chunks in this worktree.
    total_length: u32,
    /// The next available chunk ID.
    next_chunk_id: u64,
    /// A map of filepaths to their corresponding chunk IDs.
    filepath_to_chunks: HashMap<Arc<Path>, HashSet<u64>>,
}
impl WorktreeTermStats {
    pub fn new(
        chunks: HashMap<u64, ChunkStats>,
        term_stats: HashMap<Arc<str>, TermStats>,
        total_length: u32,
        filepath_to_chunks: HashMap<Arc<Path>, HashSet<u64>>,
    ) -> Self {
        let next_chunk_id = chunks.keys().max().map_or(0, |&id| id + 1);
        Self {
            chunks,
            term_stats,
            total_length,
            next_chunk_id,
            filepath_to_chunks,
        }
    }
    pub fn add_chunk(&mut self, chunk: ChunkStats, filepath: Arc<Path>) -> u64 {
        let chunk_id = self.next_chunk_id;
        self.next_chunk_id += 1;

        // Update term_stats
        for (term, &freq) in &chunk.terms {
            let stats = self.term_stats.entry(term.clone()).or_insert(TermStats {
                frequency: 0,
                chunk_ids: HashSet::new(),
            });
            stats.frequency += freq;
            stats.chunk_ids.insert(chunk_id);
        }

        // Update total_length
        self.total_length += chunk.length;

        // Add chunk to chunks
        self.chunks.insert(chunk_id, chunk);

        // Update filepath_to_chunks
        self.filepath_to_chunks
            .entry(filepath)
            .or_insert_with(HashSet::new)
            .insert(chunk_id);

        chunk_id
    }
    pub fn remove_chunk(&mut self, chunk_id: u64) -> Option<ChunkStats> {
        if let Some(chunk) = self.chunks.remove(&chunk_id) {
            // Update term_stats
            for (term, &freq) in &chunk.terms {
                if let Some(stats) = self.term_stats.get_mut(term) {
                    stats.frequency -= freq;
                    stats.chunk_ids.remove(&chunk_id);
                    if stats.chunk_ids.is_empty() {
                        self.term_stats.remove(term);
                    }
                }
            }
            // Update total_length
            self.total_length -= chunk.length;

            // Update filepath_to_chunks
            self.filepath_to_chunks.retain(|_, chunk_set| {
                chunk_set.remove(&chunk_id);
                !chunk_set.is_empty()
            });

            Some(chunk)
        } else {
            None
        }
    }

    pub fn remove_file(&mut self, filepath: &Arc<Path>) {
        if let Some(chunk_ids) = self.filepath_to_chunks.remove(filepath) {
            for chunk_id in chunk_ids {
                self.remove_chunk(chunk_id);
            }
        }
    }
}

impl Bm25Scorer for WorktreeTermStats {
    fn total_chunks(&self) -> u64 {
        self.chunks.len() as u64
    }

    fn avg_chunk_length(&self) -> f32 {
        if self.chunks.is_empty() {
            0.0
        } else {
            self.total_length as f32 / self.chunks.len() as f32
        }
    }

    fn term_frequency(&self, term: &Arc<str>, chunk_id: u64) -> Option<u32> {
        self.chunks
            .get(&chunk_id)
            .and_then(|chunk| chunk.terms.get(term))
            .cloned()
    }

    fn chunk_length(&self, chunk_id: u64) -> Option<u32> {
        self.chunks.get(&chunk_id).map(|chunk| chunk.length)
    }

    fn document_frequency(&self, term: &Arc<str>) -> u64 {
        self.term_stats
            .get(term)
            .map(|stats| stats.chunk_ids.len() as u64)
            .unwrap_or(0)
    }
}
