mod error;
mod example;
mod mcp_tools;
mod store;
mod tools;
mod types;
mod utils;

use gpui::Global;
use std::path::PathBuf;
use std::sync::Arc;

pub use error::VectorStoreError;
pub use example::{run_example, run_mcp_example, DummyEmbeddingProvider};
pub use mcp_tools::{
    add_vector, create_store, delete_vector, get_vector, list_stores, search_vectors,
    update_metadata, AddVectorInput, CreateStoreInput, DeleteVectorInput, GetVectorInput,
    ListStoresInput, SearchResultOutput, SearchVectorsInput, UpdateMetadataInput,
};
pub use store::VectorStore;
pub use tools::init_tools;
pub use types::{Embedding, Metadata, VectorEntry};
pub use utils::cosine_similarity;

pub use store::Provider;

/// Initialize the vector store in the given application context.
pub fn init(db_path: PathBuf, cx: &mut gpui::App) {
    cx.set_global(VectorStoreRegistry::new(db_path));
}

/// The main registry for vector stores
pub struct VectorStoreRegistry {
    db_path: PathBuf,
    stores: collections::HashMap<String, Arc<VectorStore>>,
}

impl Global for VectorStoreRegistry {}

impl VectorStoreRegistry {
    fn new(db_path: PathBuf) -> Self {
        Self {
            db_path,
            stores: collections::HashMap::default(),
        }
    }

    /// Get or create a vector store with the given name.
    pub fn get_or_create_store(
        &mut self,
        name: &str,
        dimensions: usize,
    ) -> Result<Arc<VectorStore>, VectorStoreError> {
        if let Some(store) = self.stores.get(name) {
            return Ok(store.clone());
        }

        let mut store_path = self.db_path.clone();
        store_path.push(format!("{}.0.mdb", name));
        
        // Ensure parent directory exists
        if let Some(parent) = store_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let store = Arc::new(VectorStore::new(store_path, dimensions)?);
        self.stores.insert(name.to_string(), store.clone());
        Ok(store)
    }

    /// Get a vector store by name
    pub fn get_store(&self, name: &str) -> Option<Arc<VectorStore>> {
        self.stores.get(name).cloned()
    }

    /// Get all available store names
    pub fn list_stores(&self) -> Vec<String> {
        self.stores.keys().cloned().collect()
    }
}

/// Get the VectorStoreRegistry from the application context.
pub fn registry(cx: &gpui::App) -> &VectorStoreRegistry {
    cx.global::<VectorStoreRegistry>()
}

/// Get a mutable reference to the VectorStoreRegistry from the application context.
pub fn registry_mut(cx: &mut gpui::App) -> &mut VectorStoreRegistry {
    cx.global_mut::<VectorStoreRegistry>()
} 