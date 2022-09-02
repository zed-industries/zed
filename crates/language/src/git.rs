use std::ops::Range;
use std::sync::Arc;

use sum_tree::Bias;
use text::{Anchor, Point};

pub use git2 as libgit;
use libgit::{DiffOptions as GitOptions, Patch as GitPatch};

#[derive(Debug, Clone, Copy)]
pub enum DiffHunkStatus {
    Added,
    Modified,
    Removed,
}

#[derive(Debug)]
pub struct DiffHunk<T> {
    pub buffer_range: Range<T>,
    pub head_range: Range<usize>,
}

impl DiffHunk<u32> {
    pub fn status(&self) -> DiffHunkStatus {
        if self.head_range.is_empty() {
            DiffHunkStatus::Added
        } else if self.buffer_range.is_empty() {
            DiffHunkStatus::Removed
        } else {
            DiffHunkStatus::Modified
        }
    }
}

pub struct BufferDiff {
    hunks: Arc<[DiffHunk<Anchor>]>,
}

impl BufferDiff {
    pub fn new() -> BufferDiff {
        BufferDiff {
            hunks: Arc::new([]),
        }
    }

    pub fn hunks(&self) -> Arc<[DiffHunk<Anchor>]> {
        self.hunks.clone()
    }

    pub fn update(&mut self, head: &str, buffer: &text::BufferSnapshot) {
        let head = head.as_bytes();
        let current = buffer.as_rope().to_string().into_bytes();

        let mut options = GitOptions::default();
        options.context_lines(0);
        let patch = match GitPatch::from_buffers(head, None, &current, None, Some(&mut options)) {
            Ok(patch) => patch,
            Err(_) => {
                //Reset hunks in case of failure to avoid showing a stale (potentially erroneous) diff
                self.hunks = Arc::new([]);
                return;
            }
        };

        let mut hunks = Vec::new();
        for index in 0..patch.num_hunks() {
            let (hunk, _) = match patch.hunk(index) {
                Ok(it) => it,
                Err(_) => continue,
            };

            let new_start = hunk.new_start() - 1;
            let new_end = new_start + hunk.new_lines();
            let start_anchor = buffer.anchor_at(Point::new(new_start, 0), Bias::Left);
            let end_anchor = buffer.anchor_at(Point::new(new_end, 0), Bias::Left);
            let buffer_range = start_anchor..end_anchor;

            let old_start = hunk.old_start() as usize - 1;
            let old_end = old_start + hunk.old_lines() as usize;
            let head_range = old_start..old_end;

            hunks.push(DiffHunk {
                buffer_range,
                head_range,
            });
        }

        self.hunks = hunks.into();
    }
}

#[derive(Debug, Clone, Copy)]
pub enum GitDiffEdit {
    Added(u32),
    Modified(u32),
    Removed(u32),
}

impl GitDiffEdit {
    pub fn line(self) -> u32 {
        use GitDiffEdit::*;

        match self {
            Added(line) | Modified(line) | Removed(line) => line,
        }
    }
}
