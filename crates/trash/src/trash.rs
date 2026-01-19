#[cfg(all(
    unix,
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "android")
))]
mod linux;
#[cfg(target_os = "macos")]
mod mac;
#[cfg(windows)]
mod windows;

use std::path::{Path, PathBuf};

use anyhow::{Result, anyhow};
use collections::HashMap;
use smallvec::SmallVec;

trait Platform: Send + Sync {
    /// Move a file to the platform's trash, returning the path in trash.
    fn trash_file(&self, path: &Path) -> Result<PathBuf>;

    /// Move a directory to the platform's trash, returning the path in trash.
    fn trash_dir(&self, path: &Path) -> Result<PathBuf>;

    /// Restore a file from trash back to its original location.
    fn restore_file(&self, path_in_trash: &Path, original_path: &Path) -> Result<()>;

    /// Restore a directory from trash back to its original location.
    fn restore_dir(&self, path_in_trash: &Path, original_path: &Path) -> Result<()>;
}

pub struct Trash {
    platform: Box<dyn Platform>,
    // Duplicate items are rare we can optimize by storing on stack
    items: HashMap<PathBuf, SmallVec<[TrashItem; 1]>>,
}

#[derive(Clone)]
struct TrashItem {
    path_in_trash: PathBuf,
    is_dir: bool,
}

impl Default for Trash {
    fn default() -> Self {
        Self::new()
    }
}

impl Trash {
    pub fn new() -> Self {
        Self {
            items: HashMap::default(),
            platform: create_platform(),
        }
    }

    pub fn trash_file(&mut self, path: &Path) -> Result<()> {
        let path_in_trash = self.platform.trash_file(path)?;
        self.track_item(path, path_in_trash, false);
        Ok(())
    }

    pub fn trash_dir(&mut self, path: &Path) -> Result<()> {
        let path_in_trash = self.platform.trash_dir(path)?;
        self.track_item(path, path_in_trash, true);
        Ok(())
    }

    /// Restore the most recently trashed item with the given original path.
    pub fn restore(&mut self, original_path: &Path) -> Result<()> {
        let entries = self
            .items
            .get_mut(original_path)
            .ok_or_else(|| anyhow!("no trashed item found for path: {:?}", original_path))?;

        let entry = entries
            .pop()
            .ok_or_else(|| anyhow!("no trashed item found for path: {:?}", original_path))?;

        let result = if entry.is_dir {
            self.platform
                .restore_dir(&entry.path_in_trash, original_path)
        } else {
            self.platform
                .restore_file(&entry.path_in_trash, original_path)
        };

        if result.is_err() {
            entries.push(entry);
        }

        if entries.is_empty() {
            self.items.remove(original_path);
        }

        result
    }

    /// Returns true if there are any trashed items for the given original path.
    pub fn has_trashed_item(&self, original_path: &Path) -> bool {
        self.items
            .get(original_path)
            .is_some_and(|entries| !entries.is_empty())
    }

    /// Returns the number of trashed items for the given original path.
    pub fn trashed_item_count(&self, original_path: &Path) -> usize {
        self.items
            .get(original_path)
            .map(|entries| entries.len())
            .unwrap_or(0)
    }

    /// Clear all tracked items without restoring them.
    pub fn clear(&mut self) {
        self.items.clear();
    }

    fn track_item(&mut self, original_path: &Path, path_in_trash: PathBuf, is_dir: bool) {
        self.items
            .entry(original_path.to_path_buf())
            .or_default()
            .push(TrashItem {
                path_in_trash,
                is_dir,
            });
    }
}

#[cfg(all(
    unix,
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "android")
))]
fn create_platform() -> Box<dyn Platform> {
    Box::new(
        freedesktop::FreeDesktopPlatform::new().expect("failed to initialize FreeDesktop trash"),
    )
}

#[cfg(target_os = "macos")]
fn create_platform() -> Box<dyn Platform> {
    Box::new(mac::MacPlatform)
}

#[cfg(windows)]
fn create_platform() -> Box<dyn Platform> {
    Box::new(windows::WindowsPlatform)
}
