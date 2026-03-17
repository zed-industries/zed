use parking_lot::Mutex;
use std::sync::Arc;

use crate::{PathEvent, Watcher};

/// No-op file watcher for iOS. The thin client does not watch local files;
/// all file operations happen on the remote host.
pub struct FsWatcher;

impl FsWatcher {
    pub fn new(
        _tx: smol::channel::Sender<()>,
        _pending_path_events: Arc<Mutex<Vec<PathEvent>>>,
    ) -> Self {
        FsWatcher
    }
}

impl Watcher for FsWatcher {
    fn add(&self, _path: &std::path::Path) -> anyhow::Result<()> {
        Ok(())
    }

    fn remove(&self, _path: &std::path::Path) -> anyhow::Result<()> {
        Ok(())
    }
}
