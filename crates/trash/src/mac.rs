use std::{
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Result, anyhow};
use objc2_foundation::{NSFileManager, NSString, NSURL};

use crate::Platform;

pub struct MacPlatform;

impl Platform for MacPlatform {
    fn trash_file(&self, path: &Path) -> Result<PathBuf> {
        let path_str = path.to_string_lossy();
        let ns_string = NSString::from_str(&path_str);
        let url = unsafe { NSURL::fileURLWithPath(&ns_string) };
        let file_manager = unsafe { NSFileManager::defaultManager() };

        let mut result_url: Option<objc2::rc::Retained<NSURL>> = None;

        unsafe { file_manager.trashItemAtURL_resultingItemURL_error(&url, Some(&mut result_url)) }
            .map_err(|e| anyhow!("failed to trash item: {}", e.localizedDescription()))?;

        let trash_path = result_url
            .and_then(|url| unsafe { url.path() })
            .map(|p| PathBuf::from(p.to_string()))
            .ok_or_else(|| anyhow!("failed to get trash path"))?;

        Ok(trash_path)
    }

    fn trash_dir(&self, path: &Path) -> Result<PathBuf> {
        self.trash_file(path)
    }

    fn restore_file(&self, path_in_trash: &Path, original_path: &Path) -> Result<()> {
        if let Some(parent) = original_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::rename(path_in_trash, original_path)?;
        Ok(())
    }

    fn restore_dir(&self, path_in_trash: &Path, original_path: &Path) -> Result<()> {
        self.restore_file(path_in_trash, original_path)
    }
}
