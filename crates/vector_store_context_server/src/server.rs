use context_server::protocol::ServerCapability;
use gpui::App;
use std::path::PathBuf;
use vector_store::VectorStoreRegistry;

pub struct VectorStoreServer {
    _registry: Option<VectorStoreRegistry>,
    _db_path: PathBuf,
}

impl VectorStoreServer {
    pub fn new(db_path: PathBuf) -> Self {
        Self {
            _registry: None,
            _db_path: db_path,
        }
    }

    #[allow(dead_code)]
    pub fn initialize(&self, cx: &mut App) {
        // Initialize the vector store registry
        vector_store::init(self._db_path.clone(), cx);
    }

    #[allow(dead_code)]
    pub fn capabilities(&self) -> Vec<ServerCapability> {
        vec![ServerCapability::Tools]
    }
} 