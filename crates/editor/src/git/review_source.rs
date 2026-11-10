use super::*;
use anyhow::{Context as _, ensure};
use project::git_store::diff_buffer_list::DiffBase;
use serde::{Deserialize, Serialize};
use sha2::{Digest as _, Sha256};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InlineReviewDiff {
    pub comparison: ReviewComparison,
    pub side: ReviewSide,
    pub base_sha256: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewComparison {
    Unstaged,
    Staged,
    Uncommitted,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewSide {
    Old,
    New,
}

impl InlineReviewDiff {
    pub fn description(&self) -> String {
        let comparison = match self.comparison {
            ReviewComparison::Unstaged => "unstaged diff (index → working tree/editor snapshot)",
            ReviewComparison::Staged => "staged diff (HEAD → index)",
            ReviewComparison::Uncommitted => {
                "uncommitted diff (HEAD → working tree/editor snapshot)"
            }
        };
        let side = match self.side {
            ReviewSide::Old => "old side",
            ReviewSide::New => "new side",
        };
        format!("{comparison}, {side}")
    }
}

pub(super) struct ReviewSource {
    pub path: std::path::PathBuf,
    pub lines: Range<u32>,
    pub excerpt: String,
    pub before: String,
    pub after: String,
    pub diff: Option<InlineReviewDiff>,
}

impl Editor {
    pub(super) fn inline_review_selection(&self, cx: &App) -> Result<Range<Anchor>> {
        ensure!(
            self.selections.count() == 1,
            "Select one contiguous range in one file and one diff side"
        );
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let points = self.selections.newest_anchor().range().to_point(&snapshot);
        let end_row = if points.end.column == 0 && points.end.row > points.start.row {
            points.end.row - 1
        } else {
            points.end.row
        };
        Ok(snapshot.anchor_after(Point::new(points.start.row, 0))
            ..snapshot.anchor_before(Point::new(
                end_row,
                snapshot.line_len(MultiBufferRow(end_row)),
            )))
    }

    pub(super) fn inline_review_source(
        &self,
        range: Range<Anchor>,
        cx: &App,
    ) -> Result<ReviewSource> {
        let multibuffer = self.buffer.read(cx);
        let snapshot = multibuffer.snapshot(cx);
        ensure!(
            range.start.is_valid(&snapshot) && range.end.is_valid(&snapshot),
            "The review range is no longer available"
        );
        ensure!(
            range.start.cmp(&range.end, &snapshot).is_le(),
            "The review range changed; select the source lines again"
        );
        let (excerpt_buffer, _) = snapshot.excerpt_containing(range.clone()).context(
            "Select lines within one file excerpt; do not cross files or hidden context gaps",
        )?;
        let mut regions = snapshot.range_to_buffer_ranges_with_deleted_hunks(range.clone());
        let (buffer, mut offsets, _) = regions
            .next()
            .context("Select file lines, not a diff header")?;
        for (next_buffer, next, _) in regions {
            ensure!(
                next_buffer.remote_id() == buffer.remote_id() && offsets.end == next.start,
                "Select only one diff side; old/deleted and new/current lines cannot be mixed"
            );
            offsets.end = next.end;
        }
        ensure!(
            offsets.start <= offsets.end && offsets.end.0 <= buffer.len(),
            "Select source lines, not synthetic diff separators"
        );
        let start = buffer.offset_to_point(offsets.start.0);
        let end = buffer.offset_to_point(offsets.end.0);
        ensure!(
            start.column == 0 && end.column == buffer.line_len(end.row),
            "Select complete source lines, not synthetic diff separators"
        );
        let excerpt = buffer
            .text_for_range(offsets.start.0..offsets.end.0)
            .collect::<String>();
        ensure!(
            snapshot.text_for_range(range).collect::<String>() == excerpt,
            "The selection crosses a diff separator or omitted context"
        );
        let in_diff = !multibuffer.is_singleton() || self.has_any_expanded_diff_hunks(cx);
        let (path, diff) = if in_diff {
            let diff = multibuffer
                .diff_for(excerpt_buffer.remote_id())
                .context("This view has no supported Git comparison for the selected file")?;
            let project = self.project.as_ref().context("The diff has no project")?;
            let (main_buffer, base) = project
                .read(cx)
                .git_store()
                .read(cx)
                .source_for_diff(&diff, cx)
                .context("Only staged, unstaged, and uncommitted Git comparisons are supported")?;
            let path = main_buffer
                .read(cx)
                .file()
                .and_then(|file| file.file_system_abs_path(cx))
                .context("The diff file has no host path")?;
            let diff = diff.read(cx).snapshot(cx);
            let side = if buffer.remote_id() == diff.base_text().remote_id() {
                ReviewSide::Old
            } else {
                ensure!(
                    buffer.remote_id() == diff.buffer_snapshot().remote_id(),
                    "Selected lines do not belong to this comparison"
                );
                ReviewSide::New
            };
            let comparison = match base {
                DiffBase::Index => ReviewComparison::Unstaged,
                DiffBase::Staged => ReviewComparison::Staged,
                DiffBase::Head => ReviewComparison::Uncommitted,
                DiffBase::Merge { .. } => anyhow::bail!("Branch comparisons are not supported yet"),
            };
            (
                path,
                Some(InlineReviewDiff {
                    comparison,
                    side,
                    base_sha256: format!(
                        "{:x}",
                        Sha256::digest(diff.base_text().text().as_bytes())
                    ),
                }),
            )
        } else {
            let file = buffer.file().context("Open a saved file")?;
            ensure!(
                file.disk_state().exists(),
                "Open a saved current file or a supported Git diff"
            );
            (
                file.file_system_abs_path(cx)
                    .context("File has no host path")?,
                None,
            )
        };
        Ok(ReviewSource {
            path,
            lines: start.row..end.row + 1,
            excerpt,
            before: buffer
                .text_for_range(Point::new(start.row.saturating_sub(2), 0)..start)
                .collect(),
            after: buffer
                .text_for_range(
                    end..Point::new(end.row.saturating_add(3), 0).min(buffer.max_point()),
                )
                .collect(),
            diff,
        })
    }

    pub(super) fn inline_review_side_visible(&self, side: ReviewSide, cx: &App) -> Result<bool> {
        let range = self.inline_review_selection(cx)?;
        let multibuffer = self.buffer.read(cx);
        let snapshot = multibuffer.snapshot(cx);
        let (buffer, _) = snapshot
            .excerpt_containing(range)
            .context("Select a file excerpt")?;
        let diff = multibuffer
            .diff_for(buffer.remote_id())
            .context("The comparison is no longer available")?;
        let diff = diff.read(cx).snapshot(cx);
        let buffer_id = match side {
            ReviewSide::Old => diff.base_text().remote_id(),
            ReviewSide::New => diff.buffer_snapshot().remote_id(),
        };
        Ok(snapshot
            .range_to_buffer_ranges_with_deleted_hunks(MultiBufferOffset(0)..snapshot.len())
            .any(|(buffer, _, _)| buffer.remote_id() == buffer_id))
    }

    pub(super) fn saved_review_range(
        &self,
        review: &SavedInlineReview,
        cx: &App,
    ) -> Result<Range<Anchor>> {
        let snapshot = self.buffer.read(cx).snapshot(cx);
        let mut found: Option<Range<Anchor>> = None;
        for (buffer, visible, deleted_anchor) in
            snapshot.range_to_buffer_ranges_with_deleted_hunks(MultiBufferOffset(0)..snapshot.len())
        {
            let Some(offsets) = super::review_persistence::locate_excerpt(&buffer.text(), review)
            else {
                continue;
            };
            if offsets.start < visible.start.0 || offsets.start > visible.end.0 {
                continue;
            }
            let range = if let Some(deleted_anchor) = deleted_anchor {
                if offsets.end > visible.end.0 {
                    continue;
                }
                deleted_anchor.with_diff_base_anchor(buffer.anchor_after(offsets.start))
                    ..deleted_anchor.with_diff_base_anchor(buffer.anchor_before(offsets.end))
            } else {
                let Some(range) = snapshot.buffer_anchor_range_to_anchor_range(
                    buffer.anchor_after(offsets.start)..buffer.anchor_before(offsets.end),
                ) else {
                    continue;
                };
                range
            };
            let Ok(source) = self.inline_review_source(range.clone(), cx) else {
                continue;
            };
            if source.path != review.path
                || source.diff != review.diff
                || source.excerpt != review.excerpt
            {
                continue;
            }
            if let Some(previous) = &found {
                ensure!(
                    range.to_point(&snapshot) == previous.to_point(&snapshot),
                    "Ambiguous discussion — preserved in storage, not attached to another range"
                );
            } else {
                found = Some(range);
            }
        }
        found.with_context(|| format!("Outdated, ambiguous, or unavailable discussion at original line {} — preserved in storage; reopen its original file, diff side and comparison", review.start_line + 1))
    }
}
