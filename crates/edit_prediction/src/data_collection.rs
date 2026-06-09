use crate::{EditPredictionStore, StoredEvent};

use anyhow::Context as _;
use buffer_diff::BufferDiffSnapshot;
use collections::HashMap;
use gpui::{Context, Entity, Task};
use language::BufferSnapshot;
use project::{Project, ProjectPath, WorktreeId};
use std::{fmt::Write as _, ops::Range, path::Path, sync::Arc};
use text::{OffsetRangeExt, Point};
use util::rel_path::RelPath;

pub type UncommittedDiffSnapshot = Vec<(Arc<Path>, BufferSnapshot, BufferDiffSnapshot)>;
pub type UncommittedDiffResult = std::result::Result<UncommittedDiffSnapshot, Arc<anyhow::Error>>;

pub use zeta_prompt::udiff::CURSOR_POSITION_MARKER;

pub fn uncommitted_diffs_for_events(
    project: Entity<Project>,
    worktree_id: WorktreeId,
    events: Vec<StoredEvent>,
    cx: &Context<'_, EditPredictionStore>,
) -> Task<UncommittedDiffResult> {
    let git_store = project.read_with(cx, |project, _| project.git_store().clone());

    cx.spawn(async move |_store, cx| {
        let (worktree_root_name, worktree_abs_path, path_style) = project
            .read_with(cx, |project, cx| {
                let worktree = project.worktree_for_id(worktree_id, cx)?;
                let worktree = worktree.read(cx);
                let path_style = worktree.path_style();
                let root_name = RelPath::new(Path::new(worktree.root_name_str()), path_style)
                    .ok()?
                    .into_owned();
                Some((root_name, worktree.abs_path(), path_style))
            })
            .context("failed to find worktree for uncommitted diff capture")
            .map_err(Arc::new)?;

        let events_with_paths = events
            .into_iter()
            .filter_map(|stored_event| {
                let zeta_prompt::Event::BufferChange { path, .. } = stored_event.event.as_ref();
                let path = if let Ok(path) = RelPath::new(path, path_style) {
                    path.strip_prefix(&worktree_root_name).ok()?.into_arc()
                } else {
                    let path = path.strip_prefix(worktree_abs_path.as_ref()).ok()?;
                    RelPath::new(path, path_style).ok()?.into_arc()
                };
                let project_path = ProjectPath { worktree_id, path };
                let relative_path: Arc<Path> = project_path.path.as_std_path().into();
                Some((stored_event, project_path, relative_path))
            })
            .collect::<Vec<_>>();

        let mut snapshots_by_path: HashMap<Arc<Path>, (BufferSnapshot, BufferDiffSnapshot)> =
            HashMap::default();
        for (stored_event, project_path, relative_path) in events_with_paths.iter().rev() {
            if snapshots_by_path.contains_key(relative_path) {
                continue;
            }

            let buffer = project
                .update(cx, |project, cx| {
                    project.open_buffer(project_path.clone(), cx)
                })
                .await
                .context("failed to open buffer for uncommitted diff capture")
                .map_err(Arc::new)?;
            let buffer_id = buffer.read_with(cx, |buffer, _| buffer.remote_id());
            let file_context = stored_event.file_context.clone();
            let cached_diff = file_context
                .as_ref()
                .and_then(|file_context| {
                    file_context
                        .read_with(cx, |file_context, _| file_context.uncommitted_diff.clone())
                })
                // The cached diff is keyed by path, but its hunk anchors are pinned to a
                // specific buffer. If that buffer was closed and reopened, `open_buffer`
                // hands back a buffer with a new `BufferId`; reusing the stale diff against
                // it would mix anchors from different buffers and panic. Drop the cache in
                // that case so the diff is recomputed for the current buffer.
                .filter(|diff| diff.read_with(cx, |diff, _| diff.buffer_id) == buffer_id);
            let diff = match cached_diff {
                Some(diff) => diff,
                None => {
                    let diff = git_store
                        .update(cx, |git_store, cx| {
                            git_store.open_uncommitted_diff(buffer.clone(), cx)
                        })
                        .await
                        .context("failed to open uncommitted diff for capture")
                        .map_err(Arc::new)?;
                    if let Some(file_context) = file_context {
                        file_context.update(cx, |file_context, _| {
                            file_context.uncommitted_diff = Some(diff.clone());
                        });
                    }
                    diff
                }
            };

            let buffer_snapshot = buffer.update(cx, |buffer, _| buffer.snapshot());
            let diff_snapshot = diff.update(cx, |diff, cx| diff.snapshot(cx));
            snapshots_by_path.insert(relative_path.clone(), (buffer_snapshot, diff_snapshot));
        }

        let uncommitted_diff_snapshots = snapshots_by_path
            .into_iter()
            .map(|(relative_path, (snapshot, diff_snapshot))| {
                (relative_path, snapshot, diff_snapshot)
            })
            .collect();

        Ok(uncommitted_diff_snapshots)
    })
}

pub fn compute_cursor_excerpt(
    snapshot: &language::BufferSnapshot,
    cursor_anchor: language::Anchor,
) -> (String, usize, Range<Point>) {
    use text::ToOffset as _;
    use text::ToPoint as _;

    let cursor_offset = cursor_anchor.to_offset(snapshot);
    let (excerpt_point_range, excerpt_offset_range, cursor_offset_in_excerpt) =
        crate::cursor_excerpt::compute_cursor_excerpt(snapshot, cursor_offset);
    let syntax_ranges = crate::cursor_excerpt::compute_syntax_ranges(
        snapshot,
        cursor_offset,
        &excerpt_offset_range,
    );
    let excerpt_text: String = snapshot.text_for_range(excerpt_point_range).collect();
    let (_, context_range) = zeta_prompt::compute_editable_and_context_ranges(
        &excerpt_text,
        cursor_offset_in_excerpt,
        &syntax_ranges,
        100,
        50,
    );
    let context_text = excerpt_text[context_range.clone()].to_string();
    let cursor_in_context = cursor_offset_in_excerpt.saturating_sub(context_range.start);
    let context_buffer_start =
        (excerpt_offset_range.start + context_range.start).to_point(snapshot);
    let context_buffer_end = (excerpt_offset_range.start + context_range.end).to_point(snapshot);
    (
        context_text,
        cursor_in_context,
        context_buffer_start..context_buffer_end,
    )
}

pub(crate) fn compute_uncommitted_diff(snapshot: UncommittedDiffSnapshot) -> String {
    let mut uncommitted_diff = String::new();
    let mut snapshots_by_path = snapshot;
    snapshots_by_path.sort_by(|(left_path, _, _), (right_path, _, _)| left_path.cmp(right_path));
    for (relative_path, buffer_snapshot, diff_snapshot) in snapshots_by_path {
        let base_snapshot = diff_snapshot.base_text();
        let is_existing_file = diff_snapshot.base_text_exists();

        let new_path_str = relative_path.to_string_lossy();
        let old_path_str = if is_existing_file {
            new_path_str.as_ref()
        } else {
            "/dev/null"
        };
        writeln!(
            uncommitted_diff,
            "--- {}{old_path_str}",
            if is_existing_file { "a/" } else { "" }
        )
        .ok();
        writeln!(uncommitted_diff, "+++ b/{new_path_str}").ok();

        if !is_existing_file {
            let new_text = buffer_snapshot.text();
            writeln!(
                uncommitted_diff,
                "@@ -0,0 +1,{} @@",
                new_text.lines().count()
            )
            .ok();
            for line in new_text.lines() {
                writeln!(uncommitted_diff, "+{line}").ok();
            }
            continue;
        }

        let mut ranges: Vec<(Range<u32>, Range<u32>)> = Vec::new();
        for hunk in (&diff_snapshot).hunks(&buffer_snapshot) {
            let old_start = base_snapshot
                .offset_to_point(hunk.diff_base_byte_range.start)
                .row;
            let old_end =
                exclusive_end_row(base_snapshot.offset_to_point(hunk.diff_base_byte_range.end));
            let new_start = hunk.range.start.row;
            let new_end = exclusive_end_row(hunk.range.end);
            let old_range = old_start.saturating_sub(3)..old_end + 3;
            let new_range = new_start.saturating_sub(3)..new_end + 3;

            if let Some((last_old_range, last_new_range)) = ranges.last_mut()
                && (old_range.start <= last_old_range.end || new_range.start <= last_new_range.end)
            {
                last_old_range.end = last_old_range.end.max(old_range.end);
                last_new_range.end = last_new_range.end.max(new_range.end);
                continue;
            }
            ranges.push((old_range, new_range));
        }

        for (old_range, new_range) in ranges {
            uncommitted_diff.push_str(&language::unified_diff_with_offsets(
                &base_snapshot
                    .text_for_range(
                        Point::new(old_range.start, 0)
                            ..row_start_or_max(base_snapshot, old_range.end),
                    )
                    .collect::<String>(),
                &buffer_snapshot
                    .text_for_range(
                        Point::new(new_range.start, 0)
                            ..row_start_or_max(&buffer_snapshot, new_range.end),
                    )
                    .collect::<String>(),
                old_range.start,
                new_range.start,
            ));
        }
        if !uncommitted_diff.ends_with('\n') {
            uncommitted_diff.push('\n');
        }
    }
    uncommitted_diff
}

pub(crate) fn estimate_uncommitted_diff_byte_size(snapshot: &UncommittedDiffSnapshot) -> usize {
    let mut size = 0;
    for (_, buffer_snapshot, diff_snapshot) in snapshot {
        for hunk in diff_snapshot.hunks(buffer_snapshot) {
            size += hunk.diff_base_byte_range.len();
            size += hunk.range.to_offset(buffer_snapshot).len();
        }
    }
    size
}

fn row_start_or_max(snapshot: &language::BufferSnapshot, row: u32) -> Point {
    if row >= snapshot.max_point().row {
        snapshot.max_point()
    } else {
        Point::new(row, 0)
    }
}

fn exclusive_end_row(point: Point) -> u32 {
    if point.column == 0 {
        point.row
    } else {
        point.row + 1
    }
}

pub fn format_cursor_excerpt(
    excerpt: &str,
    cursor_offset: usize,
    line_comment_prefix: &str,
) -> String {
    let cursor_line_start = excerpt[..cursor_offset]
        .rfind('\n')
        .map(|pos| pos + 1)
        .unwrap_or(0);
    let cursor_line_end = excerpt[cursor_line_start..]
        .find('\n')
        .map(|pos| cursor_line_start + pos + 1)
        .unwrap_or(excerpt.len());
    let cursor_line = &excerpt[cursor_line_start..cursor_line_end];
    let cursor_line_indent = &cursor_line[..cursor_line.len() - cursor_line.trim_start().len()];
    let cursor_column = cursor_offset - cursor_line_start;

    let mut marker_line = String::new();
    if cursor_column < line_comment_prefix.len() {
        for _ in 0..cursor_column {
            marker_line.push(' ');
        }
        marker_line.push_str(line_comment_prefix);
        write!(marker_line, " <{}", CURSOR_POSITION_MARKER).unwrap();
    } else {
        if cursor_column >= cursor_line_indent.len() + line_comment_prefix.len() {
            marker_line.push_str(cursor_line_indent);
        }
        marker_line.push_str(line_comment_prefix);
        while marker_line.len() < cursor_column {
            marker_line.push(' ');
        }
        write!(marker_line, "^{}", CURSOR_POSITION_MARKER).unwrap();
    }

    let mut result = String::with_capacity(excerpt.len() + marker_line.len() + 2);
    result.push_str(&excerpt[..cursor_line_end]);
    if !result.ends_with('\n') {
        result.push('\n');
    }
    result.push_str(&marker_line);
    if cursor_line_end < excerpt.len() {
        result.push('\n');
        result.push_str(&excerpt[cursor_line_end..]);
    }
    result
}
