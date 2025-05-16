use crate::{Embedding, Metadata, VectorEntry};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Tool for creating a new vector store
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateStoreInput {
    /// The name of the store to create
    pub name: String,
    /// The dimension of vectors to store
    pub dimensions: usize,
}

/// Tool for adding a vector to a store
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct AddVectorInput {
    /// The name of the store to add to
    pub store_name: String,
    /// The vector to add
    pub vector: Vec<f32>,
    /// Optional metadata to store with the vector
    pub metadata: Option<serde_json::Value>,
}

/// Tool for searching for similar vectors
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SearchVectorsInput {
    /// The name of the store to search
    pub store_name: String,
    /// The query vector to search for
    pub query_vector: Vec<f32>,
    /// The maximum number of results to return
    pub limit: Option<usize>,
    /// Only return results with similarity above this threshold
    pub threshold: Option<f32>,
}

/// A search result entry
#[derive(Debug, Serialize, Deserialize)]
pub struct SearchResultOutput {
    /// The ID of the entry
    pub id: String,
    /// The similarity score (0.0 to 1.0)
    pub similarity: f32,
    /// The metadata associated with the entry
    pub metadata: serde_json::Value,
}

/// Tool for getting a vector by ID
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct GetVectorInput {
    /// The name of the store to get from
    pub store_name: String,
    /// The ID of the vector to get
    pub id: String,
}

/// Tool for updating vector metadata
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct UpdateMetadataInput {
    /// The name of the store
    pub store_name: String,
    /// The ID of the vector to update
    pub id: String,
    /// The new metadata to set
    pub metadata: serde_json::Value,
}

/// Tool for deleting a vector
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeleteVectorInput {
    /// The name of the store
    pub store_name: String,
    /// The ID of the vector to delete
    pub id: String,
}

/// Tool for listing stores
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListStoresInput {
    /// Optional filter for store names
    pub filter: Option<String>,
}

/// MCP tool to create a new vector store
pub async fn create_store(input: CreateStoreInput, cx: &mut gpui::App) -> Result<String, String> {
    let registry = crate::registry_mut(cx);
    match registry.get_or_create_store(&input.name, input.dimensions) {
        Ok(_) => Ok(format!("Created store '{}' with {} dimensions", input.name, input.dimensions)),
        Err(e) => Err(format!("Error creating store: {}", e)),
    }
}

/// MCP tool to add a vector to a store
pub async fn add_vector(input: AddVectorInput, cx: &mut gpui::App) -> Result<String, String> {
    let registry = crate::registry(cx);
    let store = match registry.get_store(&input.store_name) {
        Some(s) => s,
        None => return Err(format!("Store '{}' not found", input.store_name)),
    };

    let embedding = Embedding::new(input.vector);
    let mut metadata = Metadata::new();

    if let Some(meta_value) = input.metadata {
        match Metadata::from_json(meta_value) {
            Ok(m) => metadata = m,
            Err(e) => return Err(format!("Invalid metadata: {}", e)),
        }
    }

    match store.add(embedding, metadata) {
        Ok(id) => Ok(format!("Added vector with ID: {}", id)),
        Err(e) => Err(format!("Error adding vector: {}", e)),
    }
}

/// MCP tool to search for similar vectors
pub async fn search_vectors(
    input: SearchVectorsInput,
    cx: &mut gpui::App,
) -> Result<Vec<SearchResultOutput>, String> {
    let registry = crate::registry(cx);
    let store = match registry.get_store(&input.store_name) {
        Some(s) => s,
        None => return Err(format!("Store '{}' not found", input.store_name)),
    };

    // Apply threshold if provided
    if let Some(threshold) = input.threshold {
        if let Err(e) = store.set_similarity_threshold(threshold) {
            return Err(format!("Error setting threshold: {}", e));
        }
    }

    let query = Embedding::new(input.query_vector);
    match store.search(&query, input.limit) {
        Ok(results) => {
            let output = results
                .into_iter()
                .map(|r| SearchResultOutput {
                    id: r.entry.id.to_string(),
                    similarity: r.similarity,
                    metadata: serde_json::to_value(r.entry.metadata.as_map()).unwrap_or(serde_json::Value::Null),
                })
                .collect();
            Ok(output)
        }
        Err(e) => Err(format!("Error searching vectors: {}", e)),
    }
}

/// MCP tool to get a vector by ID
pub async fn get_vector(input: GetVectorInput, cx: &mut gpui::App) -> Result<VectorEntry, String> {
    let registry = crate::registry(cx);
    let store = match registry.get_store(&input.store_name) {
        Some(s) => s,
        None => return Err(format!("Store '{}' not found", input.store_name)),
    };

    let id = match Uuid::parse_str(&input.id) {
        Ok(id) => id,
        Err(_) => return Err(format!("Invalid UUID: {}", input.id)),
    };

    match store.get(id) {
        Ok(entry) => Ok(entry),
        Err(e) => Err(format!("Error getting vector: {}", e)),
    }
}

/// MCP tool to update vector metadata
pub async fn update_metadata(
    input: UpdateMetadataInput,
    cx: &mut gpui::App,
) -> Result<String, String> {
    let registry = crate::registry(cx);
    let store = match registry.get_store(&input.store_name) {
        Some(s) => s,
        None => return Err(format!("Store '{}' not found", input.store_name)),
    };

    let id = match Uuid::parse_str(&input.id) {
        Ok(id) => id,
        Err(_) => return Err(format!("Invalid UUID: {}", input.id)),
    };

    match Metadata::from_json(input.metadata) {
        Ok(metadata) => match store.update_metadata(id, metadata) {
            Ok(_) => Ok(format!("Updated metadata for vector {}", id)),
            Err(e) => Err(format!("Error updating metadata: {}", e)),
        },
        Err(e) => Err(format!("Invalid metadata: {}", e)),
    }
}

/// MCP tool to delete a vector
pub async fn delete_vector(input: DeleteVectorInput, cx: &mut gpui::App) -> Result<String, String> {
    let registry = crate::registry(cx);
    let store = match registry.get_store(&input.store_name) {
        Some(s) => s,
        None => return Err(format!("Store '{}' not found", input.store_name)),
    };

    let id = match Uuid::parse_str(&input.id) {
        Ok(id) => id,
        Err(_) => return Err(format!("Invalid UUID: {}", input.id)),
    };

    match store.delete(id) {
        Ok(_) => Ok(format!("Deleted vector {}", id)),
        Err(e) => Err(format!("Error deleting vector: {}", e)),
    }
}

/// MCP tool to list all vector stores
pub async fn list_stores(
    input: ListStoresInput,
    cx: &mut gpui::App,
) -> Result<Vec<String>, String> {
    let registry = crate::registry(cx);
    let mut stores = registry.list_stores();
    
    if let Some(filter) = input.filter {
        stores.retain(|name| name.contains(&filter));
    }
    
    Ok(stores)
} 