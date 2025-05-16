use crate::{Embedding, Metadata, VectorStore, VectorStoreRegistry};
use anyhow::Result;
use gpui::{App, AppContext};
use std::path::PathBuf;
use std::sync::Arc;

/// Simple example demonstrating how to use the vector store
pub fn run_example(cx: &mut gpui::App) -> Result<()> {
    // Initialize the vector store
    let db_path = PathBuf::from("/tmp/vector_store_example");
    crate::init(db_path, cx);

    // Get the registry
    let registry = crate::registry_mut(cx);

    // Create a vector store
    let store = registry.get_or_create_store("example_store", 3)?;

    // Add some vectors
    let vec1 = Embedding::new(vec![1.0, 0.0, 0.0]);
    let vec2 = Embedding::new(vec![0.0, 1.0, 0.0]);
    let vec3 = Embedding::new(vec![0.0, 0.0, 1.0]);
    let vec4 = Embedding::new(vec![0.8, 0.1, 0.1]);

    // Create metadata
    let mut metadata1 = Metadata::new();
    metadata1.set("name", "Vector 1")?;
    metadata1.set("category", "red")?;

    let mut metadata2 = Metadata::new();
    metadata2.set("name", "Vector 2")?;
    metadata2.set("category", "green")?;

    let mut metadata3 = Metadata::new();
    metadata3.set("name", "Vector 3")?;
    metadata3.set("category", "blue")?;

    let mut metadata4 = Metadata::new();
    metadata4.set("name", "Vector 4")?;
    metadata4.set("category", "red")?;

    // Add vectors to the store
    let id1 = store.add(vec1, metadata1)?;
    let id2 = store.add(vec2, metadata2)?;
    let id3 = store.add(vec3, metadata3)?;
    let id4 = store.add(vec4, metadata4)?;

    println!("Added vectors with IDs: {}, {}, {}, {}", id1, id2, id3, id4);

    // Count entries
    let count = store.count()?;
    println!("Store has {} entries", count);

    // Search for similar vectors
    let query = Embedding::new(vec![0.9, 0.1, 0.0]);
    let results = store.search(&query, Some(2))?;

    println!("Search results:");
    for result in &results {
        println!(
            "  ID: {}, Similarity: {:.4}, Name: {}",
            result.entry.id,
            result.similarity,
            result.entry.metadata.get::<String>("name").unwrap_or_default()
        );
    }

    // Update metadata
    let mut updated_metadata = Metadata::new();
    updated_metadata.set("name", "Updated Vector 1")?;
    updated_metadata.set("category", "red")?;
    updated_metadata.set("updated", true)?;

    store.update_metadata(id1, updated_metadata)?;
    println!("Updated metadata for vector {}", id1);

    // Get a vector by ID
    let entry = store.get(id1)?;
    println!(
        "Retrieved vector {}: Name = {}, Updated = {}",
        id1,
        entry.metadata.get::<String>("name").unwrap_or_default(),
        entry.metadata.get::<bool>("updated").unwrap_or_default()
    );

    // Delete a vector
    store.delete(id4)?;
    println!("Deleted vector {}", id4);

    // Count entries after deletion
    let count = store.count()?;
    println!("Store now has {} entries", count);

    Ok(())
}

/// Example demonstrating how to use the MCP tools
pub fn run_mcp_example(cx: &mut gpui::App) {
    use crate::mcp_tools::*;

    // Initialize the vector store - this is just a stub for API compatibility
    let db_path = PathBuf::from("/tmp/vector_store_mcp_example");
    crate::init(db_path, cx);
    println!("MCP example initialized (stub implementation)");
}

/// Example demonstrating how to use the Provider trait for embedding generation
pub struct DummyEmbeddingProvider {
    dimensions: usize,
}

impl DummyEmbeddingProvider {
    pub fn new(dimensions: usize) -> Self {
        Self { dimensions }
    }
}

#[async_trait::async_trait]
impl crate::Provider for DummyEmbeddingProvider {
    async fn embed(&self, texts: &[String]) -> Result<Vec<Embedding>, crate::VectorStoreError> {
        // This is a dummy implementation that creates random vectors
        let embeddings = texts
            .iter()
            .map(|text| {
                // Create a deterministic vector from the text
                let mut vector = vec![0.0; self.dimensions];
                let mut sum = 0.0;
                
                for (i, c) in text.chars().enumerate() {
                    let idx = i % self.dimensions;
                    vector[idx] += c as u32 as f32 * 0.01;
                    sum += vector[idx] * vector[idx];
                }
                
                // Normalize
                if sum > 0.0 {
                    let norm = sum.sqrt();
                    for val in &mut vector {
                        *val /= norm;
                    }
                }
                
                Embedding::new(vector)
            })
            .collect();
        
        Ok(embeddings)
    }
    
    fn dimensions(&self) -> usize {
        self.dimensions
    }
} 