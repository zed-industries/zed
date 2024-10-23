use crate::ResultExt;
use async_fs as fs;
use futures_lite::StreamExt;
use std::path::{Path, PathBuf};

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

pub async fn collect_matching<F>(dir: &Path, predicate: F) -> Vec<PathBuf>
where
    F: Fn(&Path) -> bool,
{
    let mut matching = vec![];

    if let Some(mut entries) = fs::read_dir(dir).await.log_err() {
        while let Some(entry) = entries.next().await {
            if let Some(entry) = entry.log_err() {
                if predicate(entry.path().as_path()) {
                    matching.push(entry.path());
                }
            }
        }
    }

    matching
}

pub async fn find_file_name_in_dir<F>(dir: &Path, predicate: F) -> Option<PathBuf>
where
    F: Fn(&str) -> bool,
{
    if let Some(mut entries) = fs::read_dir(dir).await.log_err() {
        while let Some(entry) = entries.next().await {
            if let Some(entry) = entry.log_err() {
                let entry_path = entry.path();

                if let Some(file_name) = entry_path
                    .file_name()
                    .and_then(|file_name| file_name.to_str())
                {
                    if predicate(file_name) {
                        return Some(entry_path);
                    }
                }
            }
        }
    }

    None
}
