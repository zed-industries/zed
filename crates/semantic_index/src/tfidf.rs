use rust_stemmers::{Algorithm, Stemmer};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, sync::Arc};

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

pub type TermFrequencyMap = HashMap<Arc<str>, u32>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChunkTermFrequency(TermFrequencyMap);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorpusTermFrequency(TermFrequencyMap);

pub trait TermFrequency {
    fn add_term(&mut self, term: Arc<str>);
    fn merge(&mut self, other: &Self);
    fn subtract(&mut self, other: &Self);
    fn total_terms(&self) -> u32;
}

impl TermFrequency for TermFrequencyMap {
    fn add_term(&mut self, term: Arc<str>) {
        *self.entry(term).or_insert(0) += 1;
    }

    fn merge(&mut self, other: &Self) {
        for (term, &count) in other {
            *self.entry(term.clone()).or_insert(0) += count;
        }
    }

    fn subtract(&mut self, other: &Self) {
        for (term, &count) in other {
            if let Some(self_count) = self.get_mut(term) {
                *self_count = self_count.saturating_sub(count);
                if *self_count == 0 {
                    self.remove(term);
                }
            }
        }
    }

    fn total_terms(&self) -> u32 {
        self.values().sum()
    }
}

impl ChunkTermFrequency {
    pub fn new() -> Self {
        ChunkTermFrequency(TermFrequencyMap::new())
    }

    pub fn from_text(text: &str, tokenizer: &SimpleTokenizer) -> Self {
        let mut tf = ChunkTermFrequency::new();
        for token in tokenizer.tokenize_and_stem(text) {
            tf.0.add_term(token);
        }
        tf
    }

    pub fn update(&mut self, old_text: &str, new_text: &str, tokenizer: &SimpleTokenizer) {
        let old_tf = ChunkTermFrequency::from_text(old_text, tokenizer);
        let new_tf = ChunkTermFrequency::from_text(new_text, tokenizer);

        self.0.subtract(&old_tf.0);
        self.0.merge(&new_tf.0);
    }
}

impl CorpusTermFrequency {
    pub fn new() -> Self {
        CorpusTermFrequency(TermFrequencyMap::new())
    }

    pub fn add_chunk(&mut self, chunk: &ChunkTermFrequency) {
        self.0.merge(&chunk.0);
    }

    pub fn remove_chunk(&mut self, chunk: &ChunkTermFrequency) {
        self.0.subtract(&chunk.0);
    }

    pub fn update_chunk(&mut self, old_chunk: &ChunkTermFrequency, new_chunk: &ChunkTermFrequency) {
        self.remove_chunk(old_chunk);
        self.add_chunk(new_chunk);
    }

    pub fn document_frequency(&self, term: &Arc<str>) -> u32 {
        *self.0.get(term).unwrap_or(&0)
    }

    pub fn total_terms(&self) -> u32 {
        self.0.total_terms()
    }
}

impl std::ops::Deref for ChunkTermFrequency {
    type Target = TermFrequencyMap;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl std::ops::DerefMut for ChunkTermFrequency {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TfIdfMetadata {
    pub total_chunks: u64,
    pub document_frequencies: CorpusTermFrequency,
}

impl TfIdfMetadata {
    pub fn new() -> Self {
        Self {
            total_chunks: 0,
            document_frequencies: CorpusTermFrequency::new(),
        }
    }

    pub fn add_chunk(&mut self, chunk_term_frequencies: &ChunkTermFrequency) {
        self.total_chunks += 1;
        self.document_frequencies.add_chunk(chunk_term_frequencies);
    }

    pub fn remove_chunk(&mut self, chunk_term_frequencies: &ChunkTermFrequency) {
        self.total_chunks = self.total_chunks.saturating_sub(1);
        self.document_frequencies
            .remove_chunk(chunk_term_frequencies);
    }

    pub fn update_chunk(&mut self, old_chunk: &ChunkTermFrequency, new_chunk: &ChunkTermFrequency) {
        self.document_frequencies.update_chunk(old_chunk, new_chunk);
    }

    pub fn avg_chunk_length(&self) -> f32 {
        self.total_chunks as f32 / self.document_frequencies.total_terms() as f32
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

pub struct Bm25Calculator {
    params: Bm25Parameters,
    metadata: Arc<TfIdfMetadata>,
    avg_chunk_length: f32,
}

impl Bm25Calculator {
    pub fn new(params: Bm25Parameters, metadata: Arc<TfIdfMetadata>) -> Self {
        let avg_chunk_length = metadata.avg_chunk_length();
        Self {
            params,
            metadata,
            avg_chunk_length,
        }
    }

    pub fn calculate_score(
        &self,
        query_terms: &HashMap<Arc<str>, f32>,
        chunk: &ChunkTermFrequency,
        chunk_length: u32,
    ) -> f32 {
        query_terms
            .iter()
            .map(|(term, query_tf)| {
                let tf = self.tf_component(chunk.get(term).cloned().unwrap_or(0), chunk_length);
                let idf = self.idf_component(term);
                let norm = self.length_norm_component(chunk_length);
                query_tf * idf * (tf * norm)
            })
            .sum()
    }

    fn tf_component(&self, term_freq: u32, chunk_length: u32) -> f32 {
        let tf = term_freq as f32;
        (tf * (self.params.k1 + 1.0))
            / (tf
                + self.params.k1
                    * (1.0 - self.params.b + self.params.b * self.length_ratio(chunk_length)))
    }

    fn idf_component(&self, term: &Arc<str>) -> f32 {
        let df = self.metadata.document_frequencies.document_frequency(term) as f32;
        let n = self.metadata.total_chunks as f32;
        ((n - df + 0.5) / (df + 0.5)).ln()
    }

    fn length_norm_component(&self, chunk_length: u32) -> f32 {
        1.0 - self.params.b + self.params.b * self.length_ratio(chunk_length)
    }

    fn length_ratio(&self, chunk_length: u32) -> f32 {
        chunk_length as f32 / self.avg_chunk_length
    }
}

pub fn combine_bm25_and_embedding(bm25_score: f32, embedding_similarity: f32, alpha: f32) -> f32 {
    alpha * bm25_score + (1.0 - alpha) * embedding_similarity
}

pub fn tokenize_query(query: &str, tokenizer: &SimpleTokenizer) -> HashMap<Arc<str>, f32> {
    let tokens = tokenizer.tokenize_and_stem(query);
    let mut query_terms = HashMap::new();
    for token in tokens {
        *query_terms.entry(token).or_insert(0.0) += 1.0;
    }
    query_terms
}
