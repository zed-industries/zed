use project::WorktreeId;
use rust_stemmers::{Algorithm, Stemmer};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
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

    pub fn from_terms(terms: Vec<Arc<str>>) -> Self {
        let mut new_terms = HashMap::new();
        let length = terms.len() as u32;
        for term in terms {
            *new_terms.entry(term).or_insert(0) += 1;
        }
        ChunkStats {
            length,
            terms: new_terms,
        }
    }
}

/// Represents the term frequency statistics for a single worktree.
///
/// This struct contains information about chunks, term statistics,
/// and the total length of all chunks in the worktree.
pub struct WorktreeTermStats {
    /// The unique identifier for this worktree.
    id: WorktreeId,
    /// A map of chunk IDs to their corresponding statistics.
    chunks: HashMap<u64, ChunkStats>,
    /// A map of terms to their statistics across all chunks in this worktree.
    term_stats: HashMap<Arc<str>, TermStats>,
    /// The total length of all chunks in this worktree.
    total_length: u32,
    /// The next available chunk ID.
    next_chunk_id: u64,
}
impl WorktreeTermStats {
    pub fn new(
        id: WorktreeId,
        chunks: HashMap<u64, ChunkStats>,
        term_stats: HashMap<Arc<str>, TermStats>,
        total_length: u32,
    ) -> Self {
        let next_chunk_id = chunks.keys().max().map_or(0, |&id| id + 1);
        Self {
            id,
            chunks,
            term_stats,
            total_length,
            next_chunk_id,
        }
    }

    pub fn add_chunk(&mut self, chunk: ChunkStats) -> u64 {
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
            Some(chunk)
        } else {
            None
        }
    }
    pub fn update_chunk(&mut self, chunk_id: u64, new_chunk: ChunkStats) -> u64 {
        if let Some(old_chunk) = self.chunks.get(&chunk_id) {
            // Remove old chunk statistics
            for (term, &freq) in &old_chunk.terms {
                if let Some(stats) = self.term_stats.get_mut(term) {
                    stats.frequency -= freq;
                    stats.chunk_ids.remove(&chunk_id);
                    if stats.chunk_ids.is_empty() {
                        self.term_stats.remove(term);
                    }
                }
            }
            self.total_length -= old_chunk.length;
        }

        for (term, &freq) in &new_chunk.terms {
            let stats = self.term_stats.entry(term.clone()).or_insert(TermStats {
                frequency: 0,
                chunk_ids: HashSet::new(),
            });
            stats.frequency += freq;
            stats.chunk_ids.insert(chunk_id);
        }
        self.total_length += new_chunk.length;

        self.chunks.insert(chunk_id, new_chunk);
        chunk_id
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
