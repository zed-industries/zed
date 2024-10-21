use collections::HashMap;
use std::{path::Path, sync::Arc, time::SystemTime};

const MAX_FILES_BEFORE_RESUMMARIZE: usize = 4;
const MAX_BYTES_BEFORE_RESUMMARIZE: u64 = 1_000_000; // 1 MB

#[derive(Default, Debug)]
pub struct SummaryBacklog {
    /// Key: path to a file that needs summarization, but that we haven't summarized yet. Value: that file's size on disk, in bytes, and its mtime.
    files: HashMap<Arc<Path>, (u64, Option<SystemTime>)>,
    /// Cache of the sum of all values in `files`, so we don't have to traverse the whole map to check if we're over the byte limit.
    total_bytes: u64,
}

impl SummaryBacklog {
    /// Store the given path in the backlog, along with how many bytes are in it.
    pub fn insert(&mut self, path: Arc<Path>, bytes_on_disk: u64, mtime: Option<SystemTime>) {
        let (prev_bytes, _) = self
            .files
            .insert(path, (bytes_on_disk, mtime))
            .unwrap_or_default(); // Default to 0 prev_bytes

        // Update the cached total by subtracting out the old amount and adding the new one.
        self.total_bytes = self.total_bytes - prev_bytes + bytes_on_disk;
    }

    /// Returns true if the total number of bytes in the backlog exceeds a predefined threshold.
    pub fn needs_drain(&self) -> bool {
        self.files.len() > MAX_FILES_BEFORE_RESUMMARIZE ||
        // The whole purpose of the cached total_bytes is to make this comparison cheap.
        // Otherwise we'd have to traverse the entire dictionary every time we wanted this answer.
        self.total_bytes > MAX_BYTES_BEFORE_RESUMMARIZE
    }

    /// Remove all the entries in the backlog and return the file paths as an iterator.
    #[allow(clippy::needless_lifetimes)] // Clippy thinks this 'a can be elided, but eliding it gives a compile error
    pub fn drain<'a>(&'a mut self) -> impl Iterator<Item = (Arc<Path>, Option<SystemTime>)> + 'a {
        self.total_bytes = 0;

        self.files
            .drain()
            .map(|(path, (_size, mtime))| (path, mtime))
    }

    pub fn len(&self) -> usize {
        self.files.len()
    }
}
