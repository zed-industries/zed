use std::ops::Range;
use std::sync::Arc;

use sum_tree::Bias;
use text::{Anchor, Point};

pub use git2 as libgit;
use libgit::Patch as GitPatch;

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
        let current = buffer.as_rope().to_string().into_bytes();
        let patch = match GitPatch::from_buffers(head.as_bytes(), None, &current, None, None) {
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
                Err(_) => break,
            };

            let new_start = hunk.new_start();
            let new_end = new_start + hunk.new_lines();
            let start_anchor = buffer.anchor_at(Point::new(new_start, 0), Bias::Left);
            let end_anchor = buffer.anchor_at(Point::new(new_end, 0), Bias::Left);
            let buffer_range = start_anchor..end_anchor;

            let old_start = hunk.old_start() as usize;
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

// struct DiffTracker {
//     track_line_num: u32,
//     edits: Vec<GitDiffEdit>,
// }

// impl DiffTracker {
//     fn new() -> DiffTracker {
//         DiffTracker {
//             track_line_num: 0,
//             edits: Vec::new(),
//         }
//     }

//     fn attempt_finalize_file(&mut self, base_path: &Path) -> Result<()> {
//         let relative = if let Some(relative) = self.last_file_path.clone() {
//             relative
//         } else {
//             return Ok(());
//         };

//         let mut path = base_path.to_path_buf();
//         path.push(relative);
//         path = canonicalize(path).map_err(Error::Io)?;

//         self.diffs.push(GitFileDiff {
//             path,
//             edits: take(&mut self.edits),
//         });

//         Ok(())
//     }

//     fn handle_diff_line(
//         &mut self,
//         delta: DiffDelta,
//         line: DiffLine,
//         base_path: &Path,
//     ) -> Result<()> {
//         let path = match (delta.old_file().path(), delta.new_file().path()) {
//             (Some(old), _) => old,
//             (_, Some(new)) => new,
//             (_, _) => return Err(Error::DeltaMissingPath),
//         };

//         if self.last_file_path.as_deref() != Some(path) {
//             self.attempt_finalize_file(base_path)?;
//             self.last_file_path = Some(path.to_path_buf());
//             self.track_line_num = 0;
//         }

//         match line.origin_value() {
//             DiffLineType::Context => {
//                 self.track_line_num = line.new_lineno().ok_or(Error::ContextMissingLineNum)?;
//             }

//             DiffLineType::Deletion => {
//                 self.track_line_num += 1;
//                 self.edits.push(GitDiffEdit::Removed(self.track_line_num));
//             }

//             DiffLineType::Addition => {
//                 let addition_line_num = line.new_lineno().ok_or(Error::AdditionMissingLineNum)?;
//                 self.track_line_num = addition_line_num;

//                 let mut replaced = false;
//                 for rewind_index in (0..self.edits.len()).rev() {
//                     let edit = &mut self.edits[rewind_index];

//                     if let GitDiffEdit::Removed(removed_line_num) = *edit {
//                         match removed_line_num.cmp(&addition_line_num) {
//                             Ordering::Equal => {
//                                 *edit = GitDiffEdit::Modified(addition_line_num);
//                                 replaced = true;
//                                 break;
//                             }

//                             Ordering::Greater => continue,
//                             Ordering::Less => break,
//                         }
//                     }
//                 }

//                 if !replaced {
//                     self.edits.push(GitDiffEdit::Added(addition_line_num));
//                 }
//             }

//             _ => {}
//         }

//         Ok(())
//     }
// }
