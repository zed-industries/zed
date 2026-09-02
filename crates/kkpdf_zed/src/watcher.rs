//! File system change watcher and state-preserving auto-reload manager.
//!
//! Inspired by LaTeX-Workshop's continuous preview workflow:
//! When background compilers (e.g., `latexmk`, `pdflatex`, `typst watch`) rewrite
//! the target PDF, this module debounces file modification events and reloads
//! the document while strictly preserving:
//! 1. Current viewport scroll position & scroll percentage.
//! 2. Active zoom factor & layout mode.
//! 3. Active page index and selection states.

use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Snapshot of viewer position to restore seamlessly across document reloads.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ViewerStateSnapshot {
    /// Active page index (0-indexed).
    pub current_page: usize,
    /// Vertical scroll offset as a percentage of total document height (0.0 to 1.0).
    pub scroll_percent_y: f32,
    /// Horizontal scroll offset as a percentage of document width (0.0 to 1.0).
    pub scroll_percent_x: f32,
    /// Exact zoom level at the moment of reload.
    pub zoom_level: f32,
    /// User pan offset in pixels.
    pub pan_x: f32,
    pub pan_y: f32,
}

impl Default for ViewerStateSnapshot {
    fn default() -> Self {
        Self {
            current_page: 0,
            scroll_percent_y: 0.0,
            scroll_percent_x: 0.0,
            zoom_level: 1.0,
            pan_x: 0.0,
            pan_y: 0.0,
        }
    }
}

/// Debounced change detector for watching PDF file modifications on disk.
#[derive(Debug)]
pub struct PdfReloadDebouncer {
    file_path: PathBuf,
    debounce_duration: Duration,
    last_modified: Option<Instant>,
    last_file_mtime: Option<std::time::SystemTime>,
    is_reloading: Arc<AtomicBool>,
}

impl PdfReloadDebouncer {
    /// Creates a new debouncer for the given file path.
    pub fn new(file_path: PathBuf, debounce_ms: u64) -> Self {
        let last_file_mtime = std::fs::metadata(&file_path)
            .and_then(|m| m.modified())
            .ok();

        Self {
            file_path,
            debounce_duration: Duration::from_millis(debounce_ms),
            last_modified: None,
            last_file_mtime,
            is_reloading: Arc::new(AtomicBool::new(false)),
        }
    }

    /// Checks if the file has been modified on disk and if the debounce window has elapsed.
    /// Returns `true` when it is safe and ready to reload the document.
    pub fn should_reload(&mut self) -> bool {
        let current_mtime = match std::fs::metadata(&self.file_path).and_then(|m| m.modified()) {
            Ok(mtime) => mtime,
            Err(_) => return false, // File might be temporarily locked or undergoing atomic replacement
        };

        let now = Instant::now();

        if Some(current_mtime) != self.last_file_mtime {
            // Modification detected! Start/update debounce timer
            self.last_file_mtime = Some(current_mtime);
            self.last_modified = Some(now);
            return false;
        }

        if let Some(last_mod) = self.last_modified {
            if now.duration_since(last_mod) >= self.debounce_duration {
                // Debounce timer elapsed, file has settled
                self.last_modified = None;
                return true;
            }
        }

        false
    }

    /// Resets the debouncer state after successful reload.
    pub fn mark_reloaded(&mut self) {
        self.last_modified = None;
        if let Ok(mtime) = std::fs::metadata(&self.file_path).and_then(|m| m.modified()) {
            self.last_file_mtime = Some(mtime);
        }
    }

    /// Marks whether an async reload operation is currently underway.
    pub fn set_reloading(&self, reloading: bool) {
        self.is_reloading.store(reloading, Ordering::SeqCst);
    }

    /// True if an async reload is in progress.
    pub fn is_reloading(&self) -> bool {
        self.is_reloading.load(Ordering::SeqCst)
    }

    /// Returns the target file path.
    pub fn path(&self) -> &Path {
        &self.file_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_viewer_state_snapshot_defaults() {
        let state = ViewerStateSnapshot::default();
        assert_eq!(state.current_page, 0);
        assert_eq!(state.zoom_level, 1.0);
        assert_eq!(state.scroll_percent_y, 0.0);
    }

    #[test]
    fn test_debouncer_detects_modification() {
        let temp_dir = tempfile::tempdir().expect("Failed to create tempdir");
        let pdf_path = temp_dir.path().join("test.pdf");

        // Create initial file
        {
            let mut f = std::fs::File::create(&pdf_path).expect("Create file");
            writeln!(f, "%PDF-1.7 initial").expect("Write");
        }

        let mut debouncer = PdfReloadDebouncer::new(pdf_path.clone(), 10);
        assert!(!debouncer.should_reload(), "No modification yet");

        // Small sleep to ensure mtime timestamp differs on fast filesystems
        std::thread::sleep(Duration::from_millis(20));

        // Modify file
        {
            let mut f = std::fs::File::create(&pdf_path).expect("Modify file");
            writeln!(f, "%PDF-1.7 modified").expect("Write");
        }

        // First check detects mtime change and arms timer
        let trigger1 = debouncer.should_reload();
        assert!(!trigger1, "Should debounce immediately on change");

        // Wait for debounce window
        std::thread::sleep(Duration::from_millis(20));

        let trigger2 = debouncer.should_reload();
        assert!(trigger2, "Should trigger reload after debounce duration");

        debouncer.mark_reloaded();
        assert!(
            !debouncer.should_reload(),
            "Should not trigger again without change"
        );
    }
}
