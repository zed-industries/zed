use crate::mcp_tools::*;
use gpui::AppContext;
use std::path::PathBuf;

/// Initialize the vector store
pub fn init_tools(db_path: PathBuf, cx: &mut gpui::App) {
    // Initialize the vector store
    crate::init(db_path, cx);
} 