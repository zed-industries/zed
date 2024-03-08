use std::path::Path;

use smol::{fs, stream::StreamExt};

use crate::ResultExt;

/// Removes all files and directories matching the given predicate
pub async fn remove_matching<F>(dir: &Path, predicate: F)
where
    F: Fn(&Path) -> bool,
{
    if let Some(mut entries) = fs::read_dir(dir).await.log_err() {
        while let Some(entry) = entries.next().await {
            if let Some(entry) = entry.log_err() {
                let entry_path = entry.path();
                if predicate(entry_path.as_path()) {
                    if let Ok(metadata) = fs::metadata(&entry_path).await {
                        if metadata.is_file() {
                            fs::remove_file(&entry_path).await.log_err();
                        } else {
                            fs::remove_dir_all(&entry_path).await.log_err();
                        }
                    }
                }
            }
        }
    }
}
