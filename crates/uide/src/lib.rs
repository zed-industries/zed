//! # Unified Intelligent Data Engine (UIDE)
//! 
//! A single, intelligent storage engine that handles all AI data types:
//! vectors, documents, structured data, graphs, and time series.
//! 
//! ## Features
//! 
//! - **Universal Data Model**: One format for all data types
//! - **Intelligent Indexing**: Automatic optimization based on usage
//! - **Smart Querying**: Combines vector, text, and graph search
//! - **Model Agnostic**: Preserves knowledge across model changes
//! 
//! ## Quick Start
//! 
//! ```rust
//! use uide::{UnifiedDataEngine, UniversalQuery};
//! 
//! # async fn example() -> anyhow::Result<()> {
//! // Create engine
//! let engine = UnifiedDataEngine::new("./data").await?;
//! 
//! // Store any data type
//! #[derive(serde::Serialize, serde::Deserialize)]
//! struct MyData { name: String, value: f64 }
//! let data = MyData { name: "test".to_string(), value: 42.0 };
//! let id = engine.store(data).await?;
//! 
//! // Query intelligently
//! let query = UniversalQuery::text_search("test");
//! let results = engine.search(query).await?;
//! # Ok(())
//! # }
//! ```

pub mod engine;
pub mod storage;
pub mod query;
pub mod index;
pub mod universal;
pub mod error;
pub mod semantic_schema;

// Core types
pub use engine::UnifiedDataEngine;
pub use universal::{UniversalRecord, UniversalContent, Relationship};
pub use query::{UniversalQuery, QueryTarget, SearchResults};
pub use error::{UideError, Result};

// ID types
pub use universal::{RecordId, DataType};

// Re-export commonly used external types
pub use chrono::{DateTime, Utc};
pub use serde::{Deserialize, Serialize};
pub use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        value: f64,
        tags: Vec<String>,
    }

    #[tokio::test]
    async fn test_basic_storage_and_retrieval() {
        let temp_dir = TempDir::new().unwrap();
        let engine = UnifiedDataEngine::new(temp_dir.path().to_string_lossy()).await.unwrap();

        let test_data = TestData {
            name: "test_item".to_string(),
            value: 42.0,
            tags: vec!["important".to_string(), "test".to_string()],
        };

        // Store data
        let id = engine.store(&test_data).await.unwrap();

        // Retrieve data
        let retrieved: Option<TestData> = engine.retrieve(id).await.unwrap();
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap(), test_data);
    }

    #[tokio::test]
    async fn test_text_search() {
        let temp_dir = TempDir::new().unwrap();
        let engine = UnifiedDataEngine::new(temp_dir.path().to_string_lossy()).await.unwrap();

        // Store some test data
        let data1 = TestData {
            name: "rust programming".to_string(),
            value: 1.0,
            tags: vec!["programming".to_string()],
        };
        let data2 = TestData {
            name: "python scripting".to_string(),
            value: 2.0,
            tags: vec!["scripting".to_string()],
        };

        engine.store(&data1).await.unwrap();
        engine.store(&data2).await.unwrap();

        // Search for "rust"
        let query = UniversalQuery::text_search("rust");
        let results = engine.search(query).await.unwrap();

        assert_eq!(results.results.len(), 1);
        // Query time is always >= 0 for unsigned integers, so just check it's present
        assert!(results.query_time_ms < 10000); // Reasonable upper bound instead
    }

    #[tokio::test]
    async fn test_vector_search() {
        let temp_dir = TempDir::new().unwrap();
        let engine = UnifiedDataEngine::new(temp_dir.path().to_string_lossy()).await.unwrap();

        // Create vector records
        let vector1 = UniversalRecord::new(
            DataType::Vector,
            UniversalContent::Vector {
                dimensions: 3,
                values: vec![1.0, 0.0, 0.0],
                encoding: universal::VectorEncoding::Float32,
            },
        );
        let vector2 = UniversalRecord::new(
            DataType::Vector,
            UniversalContent::Vector {
                dimensions: 3,
                values: vec![0.9, 0.1, 0.0], // Similar to vector1
                encoding: universal::VectorEncoding::Float32,
            },
        );

        engine.store_record(vector1).await.unwrap();
        engine.store_record(vector2).await.unwrap();

        // Search for similar vectors
        let query = UniversalQuery::vector_search(vec![1.0, 0.0, 0.0], 0.8);
        let results = engine.search(query).await.unwrap();

        assert!(results.results.len() >= 1);
        // First result should be the exact match
        assert!(results.results[0].score > 0.8);
    }

    #[tokio::test]
    async fn test_engine_stats() {
        let temp_dir = TempDir::new().unwrap();
        let engine = UnifiedDataEngine::new(temp_dir.path().to_string_lossy()).await.unwrap();

        // Initially empty
        let stats = engine.stats().await.unwrap();
        assert_eq!(stats.storage.record_count, 0);

        // Add some data
        let test_data = TestData {
            name: "test".to_string(),
            value: 1.0,
            tags: vec!["test".to_string()],
        };
        engine.store(&test_data).await.unwrap();

        // Check stats
        let stats = engine.stats().await.unwrap();
        assert_eq!(stats.storage.record_count, 1);
        assert!(stats.index.text_terms_count > 0);
    }

    #[tokio::test]
    async fn test_delete_record() {
        let temp_dir = TempDir::new().unwrap();
        let engine = UnifiedDataEngine::new(temp_dir.path().to_string_lossy()).await.unwrap();

        let test_data = TestData {
            name: "to_be_deleted".to_string(),
            value: 99.0,
            tags: vec!["temporary".to_string()],
        };

        // Store and then delete
        let id = engine.store(&test_data).await.unwrap();
        let deleted = engine.delete(id).await.unwrap();
        assert!(deleted);

        // Verify deletion
        let retrieved: Option<TestData> = engine.retrieve(id).await.unwrap();
        assert!(retrieved.is_none());
    }

    #[tokio::test]
    async fn test_universal_record_creation() {
        // Test document record
        let doc_record = UniversalRecord::new(
            DataType::Document,
            UniversalContent::Document {
                text: "This is a test document".to_string(),
                tokens: None,
                language: Some("en".to_string()),
            },
        );

        assert_eq!(doc_record.data_type, DataType::Document);
        assert!(doc_record.content.searchable_text().is_some());

        // Test structured record with builder
        let structured_content = universal::StructuredBuilder::new()
            .text_field("title", "Test Title")
            .number_field("score", 95.5)
            .bool_field("active", true)
            .build();

        let struct_record = UniversalRecord::new(DataType::Structured, structured_content);
        assert_eq!(struct_record.data_type, DataType::Structured);
    }
}
