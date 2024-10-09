use rust_stemmers::{Algorithm, Stemmer};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};
use stop_words;

#[derive(Clone)]
pub struct SimpleTokenizer {
    stemmer: Arc<Stemmer>,
    stopwords: Vec<String>,
}

impl SimpleTokenizer {
    pub fn new() -> Self {
        Self {
            // TODO: handle non-English
            stemmer: Arc::new(Stemmer::create(Algorithm::English)),
            stopwords: stop_words::get(stop_words::LANGUAGE::English),
        }
    }

    pub fn tokenize_and_stem(&self, text: &str) -> Vec<Arc<str>> {
        // Split on whitespace and punctuation
        text.split(|c: char| c.is_whitespace() || c.is_ascii_punctuation())
            .flat_map(|word| {
                word.chars().fold(vec![String::new()], |mut acc, c| {
                    // Split CamelCaps and camelCase
                    if c.is_uppercase()
                        && !acc.is_empty()
                        && acc
                            .last()
                            .and_then(|s| s.chars().last())
                            .map_or(false, |last_char| last_char.is_lowercase())
                    {
                        acc.push(String::new());
                    }
                    acc.last_mut()
                        .unwrap_or(&mut String::new())
                        .push(c.to_lowercase().next().unwrap());
                    acc
                })
            })
            .filter(|s| !s.is_empty() && !self.stopwords.contains(s))
            .map(|word| {
                // Stem each word and convert to Arc<str>
                let stemmed = self.stemmer.stem(&word).to_string();
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
    fn total_chunks(&self) -> u32;
    fn avg_chunk_length(&self) -> f32;
    fn term_frequency(&self, term: &Arc<str>, chunk_term_counts: &HashMap<Arc<str>, u32>) -> u32;
    fn document_frequency(&self, term: &Arc<str>) -> u32;

    fn calculate_bm25_score(
        &self,
        query_terms: &HashMap<Arc<str>, u32>,
        chunk_terms: &HashMap<Arc<str>, u32>,
        k1: f32,
        b: f32,
    ) -> f32 {
        // average doc length, current doc length, total docs
        let avg_dl = self.avg_chunk_length();
        let dl = chunk_terms.values().sum::<u32>() as f32;
        let dn = self.total_chunks() as f32;

        query_terms
            .iter()
            .map(|(term, &query_tf)| {
                let tf = self.term_frequency(term, chunk_terms) as f32;
                let df = self.document_frequency(term) as f32;
                let idf = ((dn - df + 0.5) / (df + 0.5) + 1.0).ln();
                let numerator = tf * (k1 + 1.0);
                let denominator = tf + k1 * (1.0 - b + b * dl / avg_dl);

                idf * (numerator / denominator)
            })
            .sum()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TermCounts(pub HashMap<Arc<str>, u32>);

impl TermCounts {
    pub fn from_text(text: &str, tokenizer: &SimpleTokenizer) -> Self {
        let tokens = tokenizer.tokenize_and_stem(text);
        let mut terms = HashMap::new();

        for token in tokens {
            *terms.entry(token).or_insert(0) += 1;
        }

        TermCounts(terms)
    }
}

/// Represents the term frequency statistics for a single worktree.
///
/// This struct contains information about chunks, term statistics,
/// and the total length of all chunks in the worktree.
#[derive(Debug)]
pub struct WorktreeTermStats {
    /// A map of terms to their counts across all chunks in this worktree.
    term_counts: HashMap<Arc<str>, u32>,
    /// The total length of all chunks in this worktree.
    total_length: u32,
    /// The total number of chunks tracked in this worktree.
    total_chunks: u32,
}

impl WorktreeTermStats {
    pub fn new(term_counts: HashMap<Arc<str>, u32>, total_length: u32, total_chunks: u32) -> Self {
        Self {
            term_counts,
            total_length,
            total_chunks,
        }
    }

    pub fn add_counts(&mut self, chunk_counts: &TermCounts) {
        let mut chunk_length = 0;
        for (term, &freq) in &chunk_counts.0 {
            let counts = self.term_counts.entry(term.clone()).or_insert(0);
            *counts += freq;
            chunk_length += freq;
        }
        self.total_length += chunk_length;
        self.total_chunks += 1;
    }

    pub fn remove_counts(&mut self, chunk_counts: &TermCounts) -> () {
        debug_assert!(chunk_counts.0.len() <= self.term_counts.len());
        debug_assert!(chunk_counts
            .0
            .keys()
            .all(|k| self.term_counts.contains_key(k)));

        let mut chunk_length = 0;
        for (term, &freq) in &chunk_counts.0 {
            if let Some(stats) = self.term_counts.get_mut(term) {
                *stats -= freq;
                chunk_length += 0;
            }
        }
        self.total_length -= chunk_length;
        self.total_chunks -= 1;
    }
}

impl Bm25Scorer for WorktreeTermStats {
    fn total_chunks(&self) -> u32 {
        self.total_chunks
    }

    fn avg_chunk_length(&self) -> f32 {
        if self.total_chunks == 0 {
            0.0
        } else {
            self.total_length as f32 / self.total_chunks as f32
        }
    }

    fn term_frequency(&self, term: &Arc<str>, chunk_term_counts: &HashMap<Arc<str>, u32>) -> u32 {
        *chunk_term_counts.get(term).unwrap_or(&0)
    }

    fn document_frequency(&self, term: &Arc<str>) -> u32 {
        *self.term_counts.get(term).unwrap_or(&0)
    }
}
