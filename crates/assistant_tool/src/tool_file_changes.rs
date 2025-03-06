use std::{path::PathBuf, sync::Arc};

use collections::HashMap;
use gpui::Global;
use parking_lot::Mutex;
use ui::App;

// Accumulates file changes made during script execution.
pub struct ToolFileChanges {
    // Assistant thread ID that these files changes are associated with. Only file changes for one
    // thread are supported to avoid the need for dropping these when the associated `Thread` is
    // dropped.
    pub thread_id: Arc<str>,
    // Map from path to file contents for files changed by script execution.
    pub file_changes: Arc<Mutex<HashMap<PathBuf, Vec<u8>>>>,
}

impl Global for ToolFileChanges {}

impl ToolFileChanges {
    pub fn get(thread_id: Arc<str>, cx: &mut App) -> Arc<Mutex<HashMap<PathBuf, Vec<u8>>>> {
        match cx.try_global::<ToolFileChanges>() {
            Some(global) if global.thread_id == thread_id => global.file_changes.clone(),
            _ => {
                let file_changes = Arc::new(Mutex::new(HashMap::default()));
                cx.set_global(ToolFileChanges {
                    thread_id,
                    file_changes: file_changes.clone(),
                });
                file_changes
            }
        }
    }
}
