use crate::error::VectorStoreError;
use crate::types::{Embedding, Metadata, SearchResult, VectorEntry};
use crate::utils;
use async_trait::async_trait;
use futures::future::BoxFuture;
use heed::types::{SerdeBincode, Str};
use heed::{Database, Env, EnvOpenOptions};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::path::Path;
use std::sync::{Arc, RwLock};
use uuid::Uuid;

/// The main vector store implementation
pub struct VectorStore {
    /// Environment for the LMDB database
    env: Env,
    /// Database for storing vector entries
    entries_db: Database<SerdeBincode<Uuid>, SerdeBincode<VectorEntry>>,
    /// Expected dimension of vectors in this store
    dimensions: usize,
    /// Configuration parameters
    config: Arc<RwLock<StoreConfig>>,
}

/// Configuration parameters for the vector store
struct StoreConfig {
    /// Default number of results to return from search
    default_limit: usize,
    /// Similarity threshold (0.0 to 1.0)
    similarity_threshold: f32,
}

impl Default for StoreConfig {
    fn default() -> Self {
        Self {
            default_limit: 10,
            similarity_threshold: 0.7,
        }
    }
}

impl VectorStore {
    /// Create a new vector store with the given path and dimensions
    pub fn new(path: impl AsRef<Path>, dimensions: usize) -> Result<Self, VectorStoreError> {
        // Create parent directory if it doesn't exist
        if let Some(parent) = path.as_ref().parent() {
            std::fs::create_dir_all(parent)?;
        }

        // Open the database environment
        const ONE_GB: usize = 1024 * 1024 * 1024;
        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(ONE_GB)
                .max_dbs(2)
                .open(path)?
        };

        // Create the database for storing vector entries
        let mut txn = env.write_txn()?;
        let entries_db = env.create_database(&mut txn, Some("vector_entries"))?;
        txn.commit()?;

        Ok(Self {
            env,
            entries_db,
            dimensions,
            config: Arc::new(RwLock::new(StoreConfig::default())),
        })
    }

    /// Add a new vector entry to the store
    pub fn add(&self, embedding: Embedding, metadata: Metadata) -> Result<Uuid, VectorStoreError> {
        // Check dimensions
        if embedding.dimension() != self.dimensions {
            return Err(VectorStoreError::DimensionMismatch {
                expected: self.dimensions,
                got: embedding.dimension(),
            });
        }

        // Create entry with new UUID
        let entry = VectorEntry::new(embedding, metadata);
        let id = entry.id;

        // Store in database
        let mut txn = self.env.write_txn()?;
        self.entries_db.put(&mut txn, &id, &entry)?;
        txn.commit()?;

        Ok(id)
    }

    /// Add a vector entry with a specific ID
    pub fn add_with_id(
        &self,
        id: Uuid,
        embedding: Embedding,
        metadata: Metadata,
    ) -> Result<(), VectorStoreError> {
        // Check dimensions
        if embedding.dimension() != self.dimensions {
            return Err(VectorStoreError::DimensionMismatch {
                expected: self.dimensions,
                got: embedding.dimension(),
            });
        }

        // Create entry with provided UUID
        let entry = VectorEntry::with_id(id, embedding, metadata);

        // Store in database
        let mut txn = self.env.write_txn()?;
        self.entries_db.put(&mut txn, &id, &entry)?;
        txn.commit()?;

        Ok(())
    }

    /// Get a vector entry by its ID
    pub fn get(&self, id: Uuid) -> Result<VectorEntry, VectorStoreError> {
        let txn = self.env.read_txn()?;
        match self.entries_db.get(&txn, &id)? {
            Some(entry) => Ok(entry),
            None => Err(VectorStoreError::NotFound(id.to_string())),
        }
    }

    /// Update a vector entry's metadata
    pub fn update_metadata(
        &self,
        id: Uuid,
        metadata: Metadata,
    ) -> Result<(), VectorStoreError> {
        let txn = self.env.read_txn()?;
        match self.entries_db.get(&txn, &id)? {
            Some(mut entry) => {
                entry.metadata = metadata;
                let mut write_txn = self.env.write_txn()?;
                self.entries_db.put(&mut write_txn, &id, &entry)?;
                write_txn.commit()?;
                Ok(())
            }
            None => Err(VectorStoreError::NotFound(id.to_string())),
        }
    }

    /// Delete a vector entry by its ID
    pub fn delete(&self, id: Uuid) -> Result<(), VectorStoreError> {
        let mut txn = self.env.write_txn()?;
        if self.entries_db.delete(&mut txn, &id)? {
            txn.commit()?;
            Ok(())
        } else {
            txn.abort();
            Err(VectorStoreError::NotFound(id.to_string()))
        }
    }

    /// Search for similar vectors
    pub fn search(
        &self,
        query: &Embedding,
        limit: Option<usize>,
    ) -> Result<Vec<SearchResult>, VectorStoreError> {
        // Check dimensions
        if query.dimension() != self.dimensions {
            return Err(VectorStoreError::DimensionMismatch {
                expected: self.dimensions,
                got: query.dimension(),
            });
        }

        let limit = limit.unwrap_or_else(|| {
            self.config
                .read()
                .map(|c| c.default_limit)
                .unwrap_or(10)
        });

        let txn = self.env.read_txn()?;
        let threshold = self
            .config
            .read()
            .map(|c| c.similarity_threshold)
            .unwrap_or(0.0);

        // Use a min-heap to maintain top results
        let mut results = BinaryHeap::with_capacity(limit);

        // Iterate through all entries
        let mut iter = self.entries_db.iter(&txn)?;
        while let Some(item) = iter.next() {
            let (_, entry) = item?;
            let similarity = query.similarity(&entry.embedding);

            // Filter by similarity threshold
            if similarity < threshold {
                continue;
            }

            // Add to results heap
            let result = SearchResult {
                entry,
                similarity,
            };

            if results.len() < limit {
                results.push(ResultItem(result));
            } else if let Some(min) = results.peek() {
                if similarity > min.0.similarity {
                    results.pop();
                    results.push(ResultItem(result));
                }
            }
        }

        // Convert heap to sorted vector
        let mut sorted_results = Vec::with_capacity(results.len());
        while let Some(ResultItem(result)) = results.pop() {
            sorted_results.push(result);
        }
        sorted_results.reverse(); // Reverse to get descending order

        Ok(sorted_results)
    }

    /// Get all entries in the store
    pub fn get_all(&self) -> Result<Vec<VectorEntry>, VectorStoreError> {
        let txn = self.env.read_txn()?;
        let mut entries = Vec::new();
        let mut iter = self.entries_db.iter(&txn)?;
        while let Some(item) = iter.next() {
            let (_, entry) = item?;
            entries.push(entry);
        }
        Ok(entries)
    }

    /// Count all entries in the store
    pub fn count(&self) -> Result<usize, VectorStoreError> {
        let txn = self.env.read_txn()?;
        let count = self.entries_db.len(&txn)?;
        Ok(count as usize)
    }

    /// Set the default search limit
    pub fn set_default_limit(&self, limit: usize) -> Result<(), VectorStoreError> {
        if let Ok(mut config) = self.config.write() {
            config.default_limit = limit;
            Ok(())
        } else {
            Err(VectorStoreError::Database(
                "Failed to acquire write lock".to_string(),
            ))
        }
    }

    /// Set the similarity threshold
    pub fn set_similarity_threshold(&self, threshold: f32) -> Result<(), VectorStoreError> {
        if threshold < 0.0 || threshold > 1.0 {
            return Err(VectorStoreError::InvalidInput(
                "Similarity threshold must be between 0.0 and 1.0".to_string(),
            ));
        }

        if let Ok(mut config) = self.config.write() {
            config.similarity_threshold = threshold;
            Ok(())
        } else {
            Err(VectorStoreError::Database(
                "Failed to acquire write lock".to_string(),
            ))
        }
    }
}

/// Helper struct for BinaryHeap to sort SearchResults by similarity
struct ResultItem(SearchResult);

impl Eq for ResultItem {}

impl PartialEq for ResultItem {
    fn eq(&self, other: &Self) -> bool {
        self.0.similarity.eq(&other.0.similarity)
    }
}

impl PartialOrd for ResultItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ResultItem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0
            .similarity
            .partial_cmp(&other.0.similarity)
            .unwrap_or(Ordering::Equal)
    }
}

/// Trait for embedding providers
#[async_trait]
pub trait Provider: Send + Sync {
    /// Generate embeddings for the given texts
    async fn embed(&self, texts: &[String]) -> Result<Vec<Embedding>, VectorStoreError>;
    
    /// Get the dimensionality of the embeddings this provider generates
    fn dimensions(&self) -> usize;
} 