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

use anyhow::Result;

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

#[derive(Clone, Debug)]
pub struct TrashItem {
    pub original_path: PathBuf,
    pub path_in_trash: PathBuf,
    pub is_dir: bool,
}

pub fn trash_file(path: &Path) -> Result<TrashItem> {
    let platform = create_platform();
    let path_in_trash = platform.trash_file(path)?;
    let original_path = path.to_path_buf();
    Ok(TrashItem {
        original_path,
        path_in_trash,
        is_dir: false,
    })
}

pub fn trash_dir(path: &Path) -> Result<TrashItem> {
    let platform = create_platform();
    let path_in_trash = platform.trash_dir(path)?;
    let original_path = path.to_path_buf();
    Ok(TrashItem {
        original_path,
        path_in_trash,
        is_dir: true,
    })
}

/// Restore the most recently trashed item with the given original path.
pub fn restore(item: &TrashItem) -> Result<()> {
    let platform = create_platform();
    let result = if item.is_dir {
        platform.restore_dir(&item.path_in_trash, &item.original_path)
    } else {
        platform.restore_file(&item.path_in_trash, &item.original_path)
    };

    result
}

#[cfg(all(
    unix,
    not(target_os = "macos"),
    not(target_os = "ios"),
    not(target_os = "android")
))]
fn create_platform() -> impl Platform {
    freedesktop::FreeDesktopPlatform::new().expect("failed to initialize FreeDesktop trash")
}

#[cfg(target_os = "macos")]
fn create_platform() -> impl Platform {
    mac::MacPlatform
}

#[cfg(windows)]
fn create_platform() -> impl Platform {
    windows::WindowsPlatform
}
