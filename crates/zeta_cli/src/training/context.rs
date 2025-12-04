use std::path::Path;

use crate::source_location::SourceLocation;

#[derive(Debug, Clone, Default, clap::ValueEnum)]
pub enum ContextType {
    #[default]
    CurrentFile,
}

const MAX_CONTEXT_SIZE: usize = 16384;

pub fn collect_context(
    context_type: &ContextType,
    worktree_dir: &Path,
    cursor: SourceLocation,
) -> String {
    let context = match context_type {
        ContextType::CurrentFile => {
            let file_path = worktree_dir.join(cursor.path.as_std_path());
            std::fs::read_to_string(&file_path).unwrap_or_default()
        }
    };

    context[..MAX_CONTEXT_SIZE].to_string()
}
