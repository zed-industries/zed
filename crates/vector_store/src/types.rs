use ndarray::Array1;
use serde::{Deserialize, Serialize, de::Error as DeError};
use std::fmt;
use uuid::Uuid;

/// Embedding vector representation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Embedding {
    dimensions: Vec<f32>,
}

impl Embedding {
    /// Create a new embedding from a vector of floats
    pub fn new(mut dimensions: Vec<f32>) -> Self {
        // Normalize the vector (L2 normalization)
        let mut norm = 0.0;
        for val in &dimensions {
            norm += val * val;
        }
        norm = norm.sqrt();
        
        if norm > 0.0 {
            for val in &mut dimensions {
                *val /= norm;
            }
        }
        
        Self { dimensions }
    }
    
    /// Get the dimension (length) of the embedding vector
    pub fn dimension(&self) -> usize {
        self.dimensions.len()
    }
    
    /// Get the raw dimensions
    pub fn raw_dimensions(&self) -> &[f32] {
        &self.dimensions
    }
    
    /// Calculate cosine similarity with another embedding
    pub fn similarity(&self, other: &Embedding) -> f32 {
        crate::utils::cosine_similarity(&self.dimensions, &other.dimensions)
    }
    
    /// Convert to ndarray format
    pub fn as_array(&self) -> Array1<f32> {
        Array1::from_vec(self.dimensions.clone())
    }
}

/// Metadata associated with a vector entry
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Metadata {
    data: serde_json::Map<String, serde_json::Value>,
}

impl Metadata {
    /// Create a new empty metadata
    pub fn new() -> Self {
        Self {
            data: serde_json::Map::new(),
        }
    }
    
    /// Set a metadata field
    pub fn set<T: Serialize>(&mut self, key: &str, value: T) -> Result<(), serde_json::Error> {
        let value = serde_json::to_value(value)?;
        self.data.insert(key.to_string(), value);
        Ok(())
    }
    
    /// Get a metadata field
    pub fn get<T: for<'de> Deserialize<'de>>(&self, key: &str) -> Option<T> {
        self.data.get(key).and_then(|v| serde_json::from_value(v.clone()).ok())
    }
    
    /// Check if a key exists
    pub fn contains_key(&self, key: &str) -> bool {
        self.data.contains_key(key)
    }
    
    /// Remove a key
    pub fn remove(&mut self, key: &str) -> Option<serde_json::Value> {
        self.data.remove(key)
    }
    
    /// Get all keys
    pub fn keys(&self) -> Vec<String> {
        self.data.keys().cloned().collect()
    }
    
    /// Convert to raw map
    pub fn as_map(&self) -> &serde_json::Map<String, serde_json::Value> {
        &self.data
    }
    
    /// Create from raw JSON value
    pub fn from_json(value: serde_json::Value) -> Result<Self, serde_json::Error> {
        match value {
            serde_json::Value::Object(map) => Ok(Self { data: map }),
            _ => Err(DeError::custom("Expected JSON object")),
        }
    }
}

/// A complete vector entry with ID, embedding and metadata
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct VectorEntry {
    pub id: Uuid,
    pub embedding: Embedding,
    pub metadata: Metadata,
}

impl VectorEntry {
    /// Create a new vector entry
    pub fn new(embedding: Embedding, metadata: Metadata) -> Self {
        Self {
            id: Uuid::new_v4(),
            embedding,
            metadata,
        }
    }
    
    /// Create a new vector entry with a specific ID
    pub fn with_id(id: Uuid, embedding: Embedding, metadata: Metadata) -> Self {
        Self {
            id,
            embedding,
            metadata,
        }
    }
}

/// Represents a search result with distance to the query
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub entry: VectorEntry,
    pub similarity: f32,
}

impl fmt::Display for SearchResult {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "SearchResult {{ id: {}, similarity: {:.4} }}",
            self.entry.id, self.similarity
        )
    }
} 