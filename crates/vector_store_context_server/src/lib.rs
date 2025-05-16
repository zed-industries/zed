mod extension;
mod server;

use std::path::Path;
use std::sync::Arc;
use serde_json;

// Re-export the extension for Zed to use
pub use crate::extension::VectorStoreExtension;

// Export the server implementation for direct use
pub use crate::server::VectorStoreServer;

// Export a function that can be called to create a new extension instance
pub fn create_extension(
    manifest: Arc<dyn std::any::Any>,
    work_dir: Arc<Path>,
) -> Arc<dyn std::any::Any> {
    let manifest_value = Arc::new(serde_json::json!({
        "name": "vector-store-context-server",
        "version": "0.1.0"
    }));
    
    Arc::new(extension::VectorStoreExtension::new(
        manifest_value,
        work_dir,
    ))
}

#[no_mangle]
pub extern "C" fn _start() {
    // This function exists for compatibility reasons
}