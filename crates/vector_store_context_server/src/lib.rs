mod extension;
mod server;

use std::path::Path;
use std::sync::Arc;

// Re-export the extension for Zed to use
pub use crate::extension::VectorStoreExtension;

// Export a function that can be called to create a new extension instance
pub fn create_extension(
    _manifest: Arc<dyn std::any::Any>,
    work_dir: Arc<Path>,
) -> Arc<dyn std::any::Any> {
    // Create a dummy JSON value to pass to the extension
    let dummy_manifest = Arc::new(serde_json::json!({
        "name": "vector-store-context-server",
        "version": "0.1.0"
    }));
    
    Arc::new(extension::VectorStoreExtension::new(
        dummy_manifest,
        work_dir,
    ))
}

#[no_mangle]
pub extern "C" fn _start() {
    // The actual extension loading is now handled by Zed
    // This function exists for compatibility reasons
} 