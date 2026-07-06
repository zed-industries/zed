use collections::{HashMap, HashSet};
use gpui::{App, AppContext as _, AsyncApp, Entity, EntityId};
use language::{Buffer, BufferSnapshot, Point, ToPoint as _};
use project::{Project, ProjectPath};
use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::Arc,
};
use text::Anchor;
use util::{paths::PathStyle, rel_path::RelPath};
use zeta_prompt::{ContextSource, RelatedExcerpt, RelatedFile, multi_region::is_good_block_start};

use crate::{
    bm25_context::{Bm25ContextCandidate, collect_bm25_context},
    git_log_context::build_git_log_index,
};

/// This module contains collectors for editable context:
/// excerpts or full files that are likely to be edited.
const CURSOR_CONTEXT_LINE_COUNT: u32 = 20;
const EDIT_HISTORY_CONTEXT_LINE_COUNT: u32 = 20;
const GIT_LOG_CONTEXT_LINE_COUNT: u32 = 10000;
const GIT_LOG_CONTEXT_FILE_COUNT: usize = 10;
const ORACLE_SNIPPET_MIN_CONTEXT_LINE_COUNT: u32 = 10;
const ORACLE_SNIPPET_MAX_CONTEXT_LINE_COUNT: u32 = 40;
/// How far excerpt boundaries may be nudged to land on a natural block
/// boundary, mirroring `zeta_prompt::multi_region`'s marker placement.
const BOUNDARY_SNAP_LINE_COUNT: u32 = 5;
/// Maximum number of rows between two excerpts of the same buffer that get
/// bridged into one contiguous excerpt instead of rendering an elision
/// marker between them.
const BRIDGED_GAP_LINE_COUNT: u32 = 3;

type RangesByBuffer = HashMap<EntityId, (Entity<Buffer>, Vec<EditableContextRange>)>;

#[derive(Clone)]
pub struct EditHistoryContextEntry {
    pub buffer: Entity<Buffer>,
    pub edited_range: Range<Anchor>,
}

/// A file known (from expected patches) to be edited next, used by the
/// oracle context sources when generating training data.
#[derive(Clone, Debug)]
pub struct OracleTarget {
    pub path: Arc<Path>,
    /// 0-based, end-exclusive row ranges of the expected edit hunks.
    /// Only used by `ContextSource::OracleSnippet`.
    pub row_ranges: Vec<Range<u32>>,
}

struct EditableContextRange {
    range: Range<Anchor>,
    order: usize,
    context_source: ContextSource,
}

struct ResolvedEditableContextRange {
    range: Range<Point>,
    order: usize,
    context_source: ContextSource,
}

pub async fn collect_editable_context(
    project: Entity<Project>,
    active_buffer: Entity<Buffer>,
    cursor_position: Anchor,
    edit_history: Vec<EditHistoryContextEntry>,
    oracle_targets: Vec<OracleTarget>,
    context_sources: Vec<ContextSource>,
    cx: &mut AsyncApp,
) -> anyhow::Result<Vec<RelatedFile>> {
    let mut ranges_by_buffer = RangesByBuffer::default();

    if context_sources.contains(&ContextSource::CursorExcerpt) {
        collect_cursor_excerpt_context(
            &mut ranges_by_buffer,
            active_buffer.clone(),
            cursor_position,
            cx,
        );
    }
    if context_sources.contains(&ContextSource::CurrentFile) {
        collect_current_file_context(&mut ranges_by_buffer, active_buffer.clone(), cx);
    }
    if context_sources.contains(&ContextSource::EditHistory) {
        collect_edit_history_context(&mut ranges_by_buffer, &edit_history, cx);
    }
    if context_sources.contains(&ContextSource::EditHistoryFile) {
        collect_edit_history_file_context(&mut ranges_by_buffer, &edit_history, cx);
    }
    if context_sources.contains(&ContextSource::GitLog) {
        collect_git_log_context(
            &mut ranges_by_buffer,
            project.clone(),
            active_buffer.clone(),
            cx,
        )
        .await;
    }

    // Collected before bm25 so that, under a related-files byte budget, the
    // small snippets containing the expected edits are never trimmed away in
    // favor of bm25 excerpts.
    if context_sources.contains(&ContextSource::OracleSnippet) {
        collect_oracle_snippet_context(&mut ranges_by_buffer, project.clone(), &oracle_targets, cx)
            .await;
    }

    if context_sources.contains(&ContextSource::Bm25) {
        collect_bm25_context_ranges(
            &mut ranges_by_buffer,
            project.clone(),
            active_buffer,
            cursor_position,
            &edit_history,
            cx,
        )
        .await;
    }

    if context_sources.contains(&ContextSource::OracleFile) {
        collect_oracle_file_context(&mut ranges_by_buffer, project.clone(), &oracle_targets, cx)
            .await;
    }

    Ok(cx.update(|cx| {
        let project = project.read(cx);
        let mut related_files = ranges_by_buffer
            .into_values()
            .filter_map(|(buffer, ranges)| related_file_for_ranges(&project, &buffer, ranges, cx))
            .collect::<Vec<_>>();
        related_files.sort_by_key(|file| {
            file.excerpts
                .iter()
                .map(|excerpt| excerpt.order)
                .min()
                .unwrap_or(usize::MAX)
        });
        related_files
    }))
}

pub fn limit_retrieved_context_to_bytes(
    related_files: &[RelatedFile],
    max_bytes: usize,
) -> Vec<RelatedFile> {
    struct ExcerptCandidate {
        file_index: usize,
        excerpt_index: usize,
        order: usize,
    }

    let mut candidates = related_files
        .iter()
        .enumerate()
        .flat_map(|(file_index, file)| {
            file.excerpts
                .iter()
                .enumerate()
                .map(move |(excerpt_index, excerpt)| ExcerptCandidate {
                    file_index,
                    excerpt_index,
                    order: excerpt.order,
                })
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|candidate| {
        (
            candidate.order,
            candidate.file_index,
            candidate.excerpt_index,
        )
    });

    let mut selected_excerpts = related_files
        .iter()
        .map(|file| vec![false; file.excerpts.len()])
        .collect::<Vec<_>>();
    let mut covered_ranges_by_file = vec![Vec::<Range<u32>>::new(); related_files.len()];
    let mut selected_bytes: usize = 0;

    for candidate in candidates {
        let file = &related_files[candidate.file_index];
        let excerpt = &file.excerpts[candidate.excerpt_index];
        let added_bytes =
            uncovered_excerpt_bytes(excerpt, &covered_ranges_by_file[candidate.file_index]);
        if added_bytes == 0 || selected_bytes.saturating_add(added_bytes) > max_bytes {
            continue;
        }

        selected_bytes += added_bytes;
        selected_excerpts[candidate.file_index][candidate.excerpt_index] = true;
        push_covered_range(
            &mut covered_ranges_by_file[candidate.file_index],
            excerpt.row_range.clone(),
        );
    }

    related_files
        .iter()
        .enumerate()
        .filter_map(|(file_index, file)| {
            let excerpts = file
                .excerpts
                .iter()
                .enumerate()
                .filter_map(|(excerpt_index, excerpt)| {
                    selected_excerpts[file_index][excerpt_index].then(|| excerpt.clone())
                })
                .collect::<Vec<_>>();
            if excerpts.is_empty() {
                return None;
            }

            Some(RelatedFile {
                path: file.path.clone(),
                max_row: file.max_row,
                excerpts,
                in_open_source_repo: file.in_open_source_repo,
            })
        })
        .collect()
}

fn uncovered_excerpt_bytes(excerpt: &RelatedExcerpt, covered_ranges: &[Range<u32>]) -> usize {
    let mut bytes = 0;

    for (row, line) in (excerpt.row_range.start..).zip(excerpt.text.split_inclusive('\n')) {
        if row >= excerpt.row_range.end {
            break;
        }
        if !covered_ranges
            .iter()
            .any(|covered_range| covered_range.contains(&row))
        {
            bytes += line.len();
        }
    }

    bytes
}

fn push_covered_range(covered_ranges: &mut Vec<Range<u32>>, range: Range<u32>) {
    covered_ranges.push(range);
    covered_ranges.sort_by_key(|range| (range.start, range.end));

    let mut merged_ranges: Vec<Range<u32>> = Vec::new();
    for range in covered_ranges.drain(..) {
        if let Some(last_range) = merged_ranges.last_mut()
            && range.start <= last_range.end
        {
            last_range.end = last_range.end.max(range.end);
            continue;
        }

        merged_ranges.push(range);
    }

    *covered_ranges = merged_ranges;
}

fn collect_cursor_excerpt_context(
    ranges_by_buffer: &mut RangesByBuffer,
    active_buffer: Entity<Buffer>,
    cursor_position: Anchor,
    cx: &mut AsyncApp,
) {
    let cursor_range = active_buffer.read_with(cx, |buffer, _cx| {
        let snapshot = buffer.snapshot();
        expanded_anchor_range(
            &snapshot,
            cursor_position..cursor_position,
            CURSOR_CONTEXT_LINE_COUNT,
        )
    });

    push_context_range(
        ranges_by_buffer,
        active_buffer,
        cursor_range,
        0,
        ContextSource::CursorExcerpt,
    );
}

fn collect_current_file_context(
    ranges_by_buffer: &mut RangesByBuffer,
    active_buffer: Entity<Buffer>,
    cx: &mut AsyncApp,
) {
    collect_full_buffer_context(
        ranges_by_buffer,
        active_buffer,
        0,
        ContextSource::CurrentFile,
        cx,
    );
}

fn collect_edit_history_context(
    ranges_by_buffer: &mut RangesByBuffer,
    edit_history: &[EditHistoryContextEntry],
    cx: &mut AsyncApp,
) {
    for (index, entry) in edit_history.iter().enumerate() {
        let edit_history_range = entry.buffer.read_with(cx, |buffer, _cx| {
            expanded_anchor_range(
                &buffer.snapshot(),
                entry.edited_range.clone(),
                EDIT_HISTORY_CONTEXT_LINE_COUNT,
            )
        });

        push_context_range(
            ranges_by_buffer,
            entry.buffer.clone(),
            edit_history_range,
            index + 1,
            ContextSource::EditHistory,
        );
    }
}

fn collect_edit_history_file_context(
    ranges_by_buffer: &mut RangesByBuffer,
    edit_history: &[EditHistoryContextEntry],
    cx: &mut AsyncApp,
) {
    let next_order = next_context_order(ranges_by_buffer);
    let mut seen_buffers = HashSet::default();
    let mut index = 0;

    for entry in edit_history {
        if !seen_buffers.insert(entry.buffer.entity_id()) {
            continue;
        }

        collect_full_buffer_context(
            ranges_by_buffer,
            entry.buffer.clone(),
            next_order + index,
            ContextSource::EditHistoryFile,
            cx,
        );
        index += 1;
    }
}

async fn collect_bm25_context_ranges(
    ranges_by_buffer: &mut RangesByBuffer,
    project: Entity<Project>,
    active_buffer: Entity<Buffer>,
    cursor_position: Anchor,
    edit_history: &[EditHistoryContextEntry],
    cx: &mut AsyncApp,
) {
    let next_order = next_context_order(ranges_by_buffer);
    let candidates = collect_bm25_context(
        project.clone(),
        active_buffer,
        cursor_position,
        edit_history,
        next_order,
        cx,
    )
    .await;

    for candidate in candidates {
        collect_bm25_candidate_context(ranges_by_buffer, &project, candidate, cx).await;
    }
}

async fn collect_bm25_candidate_context(
    ranges_by_buffer: &mut RangesByBuffer,
    project: &Entity<Project>,
    candidate: Bm25ContextCandidate,
    cx: &mut AsyncApp,
) {
    let buffer = match open_buffer_for_path(project, &candidate.path, cx).await {
        Ok(Some(buffer)) => buffer,
        Ok(None) => {
            log::debug!(
                "failed to find BM25 context path: {}",
                candidate.path.display()
            );
            return;
        }
        Err(error) => {
            log::debug!(
                "failed to open BM25 context path {}: {error:#}",
                candidate.path.display()
            );
            return;
        }
    };

    let Some(range) = buffer.read_with(cx, |buffer, _cx| {
        anchor_range_for_row_range(&buffer.snapshot(), candidate.row_range.clone())
    }) else {
        return;
    };

    push_context_range(
        ranges_by_buffer,
        buffer,
        range,
        candidate.order,
        ContextSource::Bm25,
    );
}

fn anchor_range_for_row_range(
    snapshot: &BufferSnapshot,
    row_range: Range<u32>,
) -> Option<Range<Anchor>> {
    if row_range.start >= row_range.end || row_range.start > snapshot.max_point().row {
        return None;
    }

    let max_point = snapshot.max_point();
    let start = snapshot.anchor_before(Point::new(row_range.start, 0));
    let end_point = if row_range.end > max_point.row {
        max_point
    } else {
        Point::new(row_range.end, 0)
    };
    let end = snapshot.anchor_after(end_point);
    Some(start..end)
}

async fn collect_oracle_file_context(
    ranges_by_buffer: &mut RangesByBuffer,
    project: Entity<Project>,
    oracle_targets: &[OracleTarget],
    cx: &mut AsyncApp,
) {
    let next_order = next_context_order(ranges_by_buffer);
    let mut seen_buffers = HashSet::default();
    let mut index = 0;

    for target in oracle_targets {
        let buffer = match open_buffer_for_path(&project, &target.path, cx).await {
            Ok(Some(buffer)) => buffer,
            Ok(None) => {
                log::debug!("failed to find oracle file path: {}", target.path.display());
                continue;
            }
            Err(error) => {
                log::debug!(
                    "failed to open oracle file path {}: {error:#}",
                    target.path.display()
                );
                continue;
            }
        };

        if !seen_buffers.insert(buffer.entity_id()) {
            continue;
        }

        collect_full_buffer_context(
            ranges_by_buffer,
            buffer,
            next_order + index,
            ContextSource::OracleFile,
            cx,
        );
        index += 1;
    }
}

async fn collect_oracle_snippet_context(
    ranges_by_buffer: &mut RangesByBuffer,
    project: Entity<Project>,
    oracle_targets: &[OracleTarget],
    cx: &mut AsyncApp,
) {
    let next_order = next_context_order(ranges_by_buffer);
    let mut index = 0;

    for target in oracle_targets {
        if target.row_ranges.is_empty() {
            continue;
        }

        let buffer = match open_buffer_for_path(&project, &target.path, cx).await {
            Ok(Some(buffer)) => buffer,
            Ok(None) => {
                log::debug!(
                    "failed to find oracle snippet path: {}",
                    target.path.display()
                );
                continue;
            }
            Err(error) => {
                log::debug!(
                    "failed to open oracle snippet path {}: {error:#}",
                    target.path.display()
                );
                continue;
            }
        };

        for row_range in &target.row_ranges {
            let Some(range) = buffer.read_with(cx, |buffer, _cx| {
                let snapshot = buffer.snapshot();
                let padding_above = oracle_snippet_padding(&target.path, row_range.start, 0);
                let padding_below = oracle_snippet_padding(&target.path, row_range.end, 1);
                let start_row = row_range.start.saturating_sub(padding_above);
                // Empty hunk row ranges (pure insertions) still cover one row.
                let core_end_row = row_range.end.max(row_range.start + 1);
                let end_row = core_end_row.saturating_add(padding_below);
                let start_row =
                    snap_start_row_to_block_boundary(&snapshot, start_row, row_range.start);
                // `end_row` is exclusive while snapping operates on the last
                // included row.
                let end_row = snap_end_row_to_block_boundary(
                    &snapshot,
                    end_row.saturating_sub(1),
                    core_end_row.saturating_sub(1),
                ) + 1;
                anchor_range_for_row_range(&snapshot, start_row..end_row)
            }) else {
                continue;
            };

            push_context_range(
                ranges_by_buffer,
                buffer.clone(),
                range,
                next_order + index,
                ContextSource::OracleSnippet,
            );
            index += 1;
        }
    }
}

/// Deterministic pseudo-random padding, so that the expected edit is not
/// always centered in the snippet (a student model could otherwise learn the
/// excerpt center as a position prior), while keeping context retrieval
/// reproducible across runs.
fn oracle_snippet_padding(path: &Path, row: u32, salt: u32) -> u32 {
    use std::hash::{Hash as _, Hasher as _};

    let mut hasher = collections::FxHasher::default();
    path.hash(&mut hasher);
    row.hash(&mut hasher);
    salt.hash(&mut hasher);
    let span =
        u64::from(ORACLE_SNIPPET_MAX_CONTEXT_LINE_COUNT - ORACLE_SNIPPET_MIN_CONTEXT_LINE_COUNT);
    ORACLE_SNIPPET_MIN_CONTEXT_LINE_COUNT + (hasher.finish() % (span + 1)) as u32
}

async fn open_buffer_for_path(
    project: &Entity<Project>,
    path: &Path,
    cx: &mut AsyncApp,
) -> anyhow::Result<Option<Entity<Buffer>>> {
    let path = path.to_path_buf();
    let path_without_prefix: PathBuf = path.components().skip(1).collect();
    let project_path = project.update(cx, |project, cx| {
        project.find_project_path(&path, cx).or_else(|| {
            if path_without_prefix.as_os_str().is_empty() {
                None
            } else {
                project.find_project_path(&path_without_prefix, cx)
            }
        })
    });

    let Some(project_path) = project_path else {
        return Ok(None);
    };

    project
        .update(cx, |project, cx| project.open_buffer(project_path, cx))
        .await
        .map(Some)
}

fn collect_full_buffer_context(
    ranges_by_buffer: &mut RangesByBuffer,
    buffer: Entity<Buffer>,
    order: usize,
    context_source: ContextSource,
    cx: &mut AsyncApp,
) {
    let range = buffer.read_with(cx, |buffer, _cx| full_file_anchor_range(&buffer.snapshot()));
    push_context_range(ranges_by_buffer, buffer, range, order, context_source);
}

fn full_file_anchor_range(snapshot: &BufferSnapshot) -> Range<Anchor> {
    let start = snapshot.anchor_before(Point::new(0, 0));
    let max_point = snapshot.max_point();
    let end = snapshot.anchor_after(max_point);
    start..end
}

fn next_context_order(ranges_by_buffer: &RangesByBuffer) -> usize {
    ranges_by_buffer
        .values()
        .flat_map(|(_, ranges)| ranges.iter().map(|range| range.order))
        .max()
        .map_or(0, |order| order + 1)
}

async fn collect_git_log_context(
    ranges_by_buffer: &mut RangesByBuffer,
    project: Entity<Project>,
    active_buffer: Entity<Buffer>,
    cx: &mut AsyncApp,
) {
    let Some((worktree_id, active_path, worktree_abs_path)) = cx.update(|cx| {
        let buffer = active_buffer.read(cx);
        let file = buffer.file()?;
        let project = project.read(cx);
        if !project.is_local() {
            return None;
        }
        let worktree = project.worktree_for_id(file.worktree_id(cx), cx)?;
        let worktree = worktree.read(cx);
        if !worktree.is_local() {
            return None;
        }
        Some((
            file.worktree_id(cx),
            file.path().clone(),
            worktree.abs_path(),
        ))
    }) else {
        return;
    };

    let index_result = cx
        .background_spawn(async move { build_git_log_index(&worktree_abs_path).await })
        .await;
    let index = match index_result {
        Ok(index) => index,
        Err(error) => {
            log::debug!("failed to build git log context index: {error:#}");
            return;
        }
    };

    let next_order = next_context_order(ranges_by_buffer);

    for (index, related_path) in index
        .get_related(active_path.as_std_path(), GIT_LOG_CONTEXT_FILE_COUNT)
        .into_iter()
        .enumerate()
    {
        let Ok(related_path) = RelPath::new(&related_path, PathStyle::Posix) else {
            continue;
        };
        let project_path = ProjectPath {
            worktree_id,
            path: related_path.into_owned().into(),
        };
        let buffer = match project
            .update(cx, |project, cx| project.open_buffer(project_path, cx))
            .await
        {
            Ok(buffer) => buffer,
            Err(error) => {
                log::debug!("failed to open git log related buffer: {error:#}");
                continue;
            }
        };

        let range = buffer.read_with(cx, |buffer, _cx| {
            let snapshot = buffer.snapshot();
            let max_row = GIT_LOG_CONTEXT_LINE_COUNT.min(snapshot.max_point().row);
            let end = snapshot.anchor_after(Point::new(max_row, snapshot.line_len(max_row)));
            snapshot.anchor_before(Point::new(0, 0))..end
        });

        push_context_range(
            ranges_by_buffer,
            buffer,
            range,
            next_order + index,
            ContextSource::GitLog,
        );
    }
}

fn expanded_anchor_range(
    snapshot: &BufferSnapshot,
    range: Range<Anchor>,
    context_line_count: u32,
) -> Range<Anchor> {
    let start = range.start.to_point(snapshot);
    let end = range.end.to_point(snapshot);
    let start_row = start.row.saturating_sub(context_line_count);
    let end_row = end
        .row
        .saturating_add(context_line_count)
        .min(snapshot.max_point().row);
    let start_row = snap_start_row_to_block_boundary(snapshot, start_row, start.row);
    let end_row = snap_end_row_to_block_boundary(snapshot, end_row, end.row);
    let start = snapshot.anchor_before(Point::new(start_row, 0));
    let end = snapshot.anchor_after(Point::new(end_row, snapshot.line_len(end_row)));
    start..end
}

/// Nudge an excerpt's first row forward (up to `BOUNDARY_SNAP_LINE_COUNT`
/// lines, never past `core_row`) so the excerpt starts at a natural block
/// boundary: preferably a good block start right after blank line(s), or
/// failing that any good block start. Mirrors the marker placement
/// heuristics of `zeta_prompt::multi_region`.
fn snap_start_row_to_block_boundary(
    snapshot: &text::BufferSnapshot,
    row: u32,
    core_row: u32,
) -> u32 {
    let limit = core_row
        .min(row.saturating_add(BOUNDARY_SNAP_LINE_COUNT))
        .min(snapshot.max_point().row);
    let mut first_good_start = None;
    for candidate in row..=limit {
        if snapshot.is_line_blank(candidate) {
            continue;
        }
        if !is_good_block_start(line_text(snapshot, candidate).trim()) {
            continue;
        }
        if candidate > 0 && snapshot.is_line_blank(candidate - 1) {
            return candidate;
        }
        if first_good_start.is_none() {
            first_good_start = Some(candidate);
        }
    }
    first_good_start.unwrap_or(row)
}

/// Nudge an excerpt's last row backward (up to `BOUNDARY_SNAP_LINE_COUNT`
/// lines, never before `core_row`) so the excerpt ends at the last non-blank
/// line before a blank line or at the end of the file.
fn snap_end_row_to_block_boundary(snapshot: &text::BufferSnapshot, row: u32, core_row: u32) -> u32 {
    let max_row = snapshot.max_point().row;
    let row = row.min(max_row);
    if row == max_row {
        return row;
    }
    let limit = core_row.max(row.saturating_sub(BOUNDARY_SNAP_LINE_COUNT));
    for candidate in (limit..=row).rev() {
        if snapshot.is_line_blank(candidate) {
            continue;
        }
        if snapshot.is_line_blank(candidate + 1) {
            return candidate;
        }
    }
    row
}

fn line_text(snapshot: &text::BufferSnapshot, row: u32) -> String {
    snapshot
        .text_for_range(Point::new(row, 0)..Point::new(row, snapshot.line_len(row)))
        .collect()
}

fn push_context_range(
    ranges_by_buffer: &mut RangesByBuffer,
    buffer: Entity<Buffer>,
    range: Range<Anchor>,
    order: usize,
    context_source: ContextSource,
) {
    ranges_by_buffer
        .entry(buffer.entity_id())
        .or_insert_with(|| (buffer.clone(), Vec::new()))
        .1
        .push(EditableContextRange {
            range,
            order,
            context_source,
        });
}

fn related_file_for_ranges(
    project: &Project,
    buffer: &Entity<Buffer>,
    ranges: Vec<EditableContextRange>,
    cx: &App,
) -> Option<RelatedFile> {
    let buffer = buffer.read(cx);
    let snapshot = buffer.snapshot();
    let file = snapshot.file()?;
    let worktree = project.worktree_for_id(file.worktree_id(cx), cx)?;
    let path: Arc<Path> = Path::new(&format!(
        "{}/{}",
        worktree.read(cx).root_name().as_unix_str(),
        file.path().as_unix_str()
    ))
    .into();

    let mut ranges = resolved_context_ranges(ranges, &snapshot);
    split_overlapping_ranges(&mut ranges);

    let excerpts = ranges
        .into_iter()
        .map(|range| RelatedExcerpt {
            row_range: range.range.start.row..range.range.end.row,
            text: snapshot
                .text_for_range(range.range)
                .collect::<String>()
                .into(),
            order: range.order,
            context_source: range.context_source,
        })
        .collect::<Vec<_>>();

    Some(RelatedFile {
        path,
        max_row: snapshot.max_point().row,
        excerpts,
        in_open_source_repo: false,
    })
}

fn resolved_context_ranges(
    ranges: Vec<EditableContextRange>,
    snapshot: &BufferSnapshot,
) -> Vec<ResolvedEditableContextRange> {
    ranges
        .into_iter()
        .filter_map(|range| {
            let start = range.range.start.to_point(snapshot);
            let end = range.range.end.to_point(snapshot);
            if start >= end {
                return None;
            }

            Some(ResolvedEditableContextRange {
                range: start..end,
                order: range.order,
                context_source: range.context_source,
            })
        })
        .collect()
}

/// Split overlapping ranges into disjoint segments instead of merging them
/// into one range. Each segment keeps the minimum order and highest-priority
/// source among the ranges covering it, and adjacent segments with equal
/// order are coalesced. This preserves priority granularity: a small
/// high-priority snippet inside a large low-priority range remains its own
/// excerpt, so byte-budget selection can retain it even when the surrounding
/// range doesn't fit. The resulting segments are disjoint and sorted by
/// position.
///
/// Ranges separated by at most `BRIDGED_GAP_LINE_COUNT` rows are bridged:
/// the small gap is attached to the preceding segment so the excerpts render
/// as one contiguous block instead of being separated by an elision marker.
fn split_overlapping_ranges(ranges: &mut Vec<ResolvedEditableContextRange>) {
    ranges.sort_by_key(|range| (range.range.start, range.range.end));
    let mut output: Vec<ResolvedEditableContextRange> = Vec::new();
    let mut cluster: Vec<ResolvedEditableContextRange> = Vec::new();
    let mut cluster_end = Point::zero();

    for range in ranges.drain(..) {
        let bridge_limit_row = row_aligned_end(cluster_end)
            .row
            .saturating_add(BRIDGED_GAP_LINE_COUNT);
        if cluster.is_empty() || range.range.start.row <= bridge_limit_row {
            cluster_end = cluster_end.max(range.range.end);
            cluster.push(range);
        } else {
            split_cluster(std::mem::take(&mut cluster), cluster_end, &mut output);
            cluster_end = range.range.end;
            cluster.push(range);
        }
    }
    if !cluster.is_empty() {
        split_cluster(cluster, cluster_end, &mut output);
    }

    *ranges = output;
}

fn split_cluster(
    cluster: Vec<ResolvedEditableContextRange>,
    cluster_end: Point,
    output: &mut Vec<ResolvedEditableContextRange>,
) {
    if cluster.len() == 1 {
        output.extend(cluster);
        return;
    }

    let cluster_start = cluster[0].range.start;
    let mut boundaries = Vec::with_capacity(cluster.len() * 2 + 1);
    boundaries.push(cluster_start);
    for range in &cluster {
        boundaries.push(range.range.start);
        boundaries.push(row_aligned_end(range.range.end));
    }
    boundaries.retain(|boundary| *boundary >= cluster_start && *boundary < cluster_end);
    boundaries.sort();
    boundaries.dedup();
    boundaries.push(cluster_end);

    for window in boundaries.windows(2) {
        let segment = window[0]..window[1];
        // The segment is attributed to the lowest-order covering range
        // (breaking ties by source priority), so that its source label is
        // consistent with the order that drives budget selection.
        let mut order_and_source: Option<(usize, ContextSource)> = None;
        for range in &cluster {
            if range.range.start <= segment.start && row_aligned_end(range.range.end) >= segment.end
            {
                let candidate = (range.order, range.context_source);
                if order_and_source.is_none_or(|(order, context_source)| {
                    (candidate.0, context_source_order(candidate.1))
                        < (order, context_source_order(context_source))
                }) {
                    order_and_source = Some(candidate);
                }
            }
        }
        let Some((order, context_source)) = order_and_source else {
            // A bridged gap between two nearby ranges: no range covers it, so
            // attach it to the preceding segment to form contiguous output.
            if let Some(last) = output.last_mut()
                && last.range.end == segment.start
            {
                last.range.end = segment.end;
            }
            continue;
        };

        if let Some(last) = output.last_mut()
            && last.range.end == segment.start
            && last.order == order
        {
            last.range.end = segment.end;
            if context_source_order(context_source) < context_source_order(last.context_source) {
                last.context_source = context_source;
            }
            continue;
        }

        output.push(ResolvedEditableContextRange {
            range: segment,
            order,
            context_source,
        });
    }
}

/// Context ranges always cover whole lines: their ends sit either at a line
/// start (column 0) or at the end of a line's content. Cutting a segment at
/// the end of a line's content would attribute the trailing newline to the
/// next segment, so nudge such ends forward to the next line start.
fn row_aligned_end(point: Point) -> Point {
    if point.column > 0 {
        Point::new(point.row + 1, 0)
    } else {
        point
    }
}

#[allow(dead_code)]
fn push_context_source(context_sources: &mut Vec<ContextSource>, context_source: ContextSource) {
    if !context_sources.contains(&context_source) {
        context_sources.push(context_source);
        context_sources.sort_by_key(|context_source| context_source_order(*context_source));
    }
}

fn context_source_order(context_source: ContextSource) -> usize {
    match context_source {
        ContextSource::Lsp => 0,
        ContextSource::CursorExcerpt => 1,
        ContextSource::CurrentFile => 2,
        ContextSource::EditHistory => 3,
        ContextSource::EditHistoryFile => 4,
        ContextSource::GitLog => 5,
        ContextSource::Bm25 => 6,
        ContextSource::OracleSnippet => 7,
        ContextSource::OracleFile => 8,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn text_snapshot(text: &str) -> text::BufferSnapshot {
        text::Buffer::new(
            text::ReplicaId::LOCAL,
            text::BufferId::new(1).unwrap(),
            text,
        )
        .snapshot()
        .clone()
    }

    fn resolved_range(
        start_row: u32,
        end_row: u32,
        order: usize,
        context_source: ContextSource,
    ) -> ResolvedEditableContextRange {
        ResolvedEditableContextRange {
            range: Point::new(start_row, 0)..Point::new(end_row, 0),
            order,
            context_source,
        }
    }

    #[test]
    fn test_split_overlapping_ranges_coalesces_equal_orders() {
        // A full-file current-file range plus edit-history windows inside it:
        // every segment is covered by the order-0 full-file range, so they
        // coalesce back into a single range with the highest-priority source.
        let mut ranges = vec![
            resolved_range(6, 46, 1, ContextSource::EditHistory),
            resolved_range(0, 281, 0, ContextSource::CurrentFile),
            resolved_range(17, 79, 2, ContextSource::EditHistory),
            resolved_range(85, 125, 3, ContextSource::EditHistory),
        ];
        split_overlapping_ranges(&mut ranges);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].range, Point::new(0, 0)..Point::new(281, 0));
        assert_eq!(ranges[0].order, 0);
        assert_eq!(ranges[0].context_source, ContextSource::CurrentFile);
    }

    #[test]
    fn test_split_overlapping_ranges_keeps_disjoint_ranges() {
        let mut ranges = vec![
            resolved_range(50, 60, 1, ContextSource::EditHistory),
            resolved_range(0, 10, 0, ContextSource::CursorExcerpt),
            resolved_range(5, 12, 2, ContextSource::EditHistory),
        ];
        split_overlapping_ranges(&mut ranges);
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0].range, Point::new(0, 0)..Point::new(10, 0));
        assert_eq!(ranges[0].order, 0);
        assert_eq!(ranges[0].context_source, ContextSource::CursorExcerpt);
        assert_eq!(ranges[1].range, Point::new(10, 0)..Point::new(12, 0));
        assert_eq!(ranges[1].order, 2);
        assert_eq!(ranges[1].context_source, ContextSource::EditHistory);
        assert_eq!(ranges[2].range, Point::new(50, 0)..Point::new(60, 0));
        assert_eq!(ranges[2].order, 1);
    }

    #[test]
    fn test_split_overlapping_ranges_keeps_adjacent_ranges_with_distinct_orders() {
        let mut ranges = vec![
            resolved_range(0, 10, 0, ContextSource::EditHistory),
            resolved_range(10, 20, 1, ContextSource::EditHistory),
        ];
        split_overlapping_ranges(&mut ranges);
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].range, Point::new(0, 0)..Point::new(10, 0));
        assert_eq!(ranges[0].order, 0);
        assert_eq!(ranges[1].range, Point::new(10, 0)..Point::new(20, 0));
        assert_eq!(ranges[1].order, 1);
    }

    #[test]
    fn test_split_overlapping_ranges_preserves_high_priority_snippet_inside_large_range() {
        // A small high-priority snippet inside a large low-priority range
        // stays its own segment, so byte-budget selection can keep it even
        // when the surrounding range doesn't fit.
        let mut ranges = vec![
            resolved_range(0, 200, 8, ContextSource::GitLog),
            resolved_range(50, 60, 2, ContextSource::OracleSnippet),
        ];
        split_overlapping_ranges(&mut ranges);
        assert_eq!(ranges.len(), 3);
        assert_eq!(ranges[0].range, Point::new(0, 0)..Point::new(50, 0));
        assert_eq!(ranges[0].order, 8);
        assert_eq!(ranges[0].context_source, ContextSource::GitLog);
        assert_eq!(ranges[1].range, Point::new(50, 0)..Point::new(60, 0));
        assert_eq!(ranges[1].order, 2);
        assert_eq!(ranges[1].context_source, ContextSource::OracleSnippet);
        assert_eq!(ranges[2].range, Point::new(60, 0)..Point::new(200, 0));
        assert_eq!(ranges[2].order, 8);
        assert_eq!(ranges[2].context_source, ContextSource::GitLog);
    }

    #[test]
    fn test_split_overlapping_ranges_aligns_mid_line_ends_to_row_starts() {
        // Ranges ending at a line's content end (column > 0) are treated as
        // covering through that whole line, so equal-order overlapping ranges
        // still coalesce into one segment.
        let mut ranges = vec![
            ResolvedEditableContextRange {
                range: Point::new(0, 0)..Point::new(10, 5),
                order: 1,
                context_source: ContextSource::EditHistory,
            },
            resolved_range(5, 20, 1, ContextSource::EditHistory),
        ];
        split_overlapping_ranges(&mut ranges);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].range, Point::new(0, 0)..Point::new(20, 0));
        assert_eq!(ranges[0].order, 1);
    }

    #[test]
    fn test_split_overlapping_ranges_bridges_small_gaps() {
        // Ranges separated by a few rows are bridged: the gap is attached to
        // the preceding segment, producing contiguous excerpts.
        let mut ranges = vec![
            resolved_range(0, 10, 0, ContextSource::EditHistory),
            resolved_range(13, 20, 1, ContextSource::Bm25),
        ];
        split_overlapping_ranges(&mut ranges);
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].range, Point::new(0, 0)..Point::new(13, 0));
        assert_eq!(ranges[0].order, 0);
        assert_eq!(ranges[0].context_source, ContextSource::EditHistory);
        assert_eq!(ranges[1].range, Point::new(13, 0)..Point::new(20, 0));
        assert_eq!(ranges[1].order, 1);
        assert_eq!(ranges[1].context_source, ContextSource::Bm25);
    }

    #[test]
    fn test_split_overlapping_ranges_bridged_equal_orders_coalesce() {
        let mut ranges = vec![
            resolved_range(0, 10, 1, ContextSource::EditHistory),
            resolved_range(12, 20, 1, ContextSource::EditHistory),
        ];
        split_overlapping_ranges(&mut ranges);
        assert_eq!(ranges.len(), 1);
        assert_eq!(ranges[0].range, Point::new(0, 0)..Point::new(20, 0));
        assert_eq!(ranges[0].order, 1);
    }

    #[test]
    fn test_split_overlapping_ranges_does_not_bridge_large_gaps() {
        let mut ranges = vec![
            resolved_range(0, 10, 0, ContextSource::EditHistory),
            resolved_range(14, 20, 1, ContextSource::Bm25),
        ];
        split_overlapping_ranges(&mut ranges);
        assert_eq!(ranges.len(), 2);
        assert_eq!(ranges[0].range, Point::new(0, 0)..Point::new(10, 0));
        assert_eq!(ranges[1].range, Point::new(14, 0)..Point::new(20, 0));
    }

    #[test]
    fn test_snap_start_row_prefers_block_start_after_blank_line() {
        let snapshot = text_snapshot(concat!(
            "    body\n",   // 0
            "}\n",          // 1
            "\n",           // 2
            "fn bar() {\n", // 3
            "    body\n",   // 4
            "    core\n",   // 5
        ));
        // Row 1 is a structural tail and row 2 is blank; row 3 follows a
        // blank line and is a good block start.
        assert_eq!(snap_start_row_to_block_boundary(&snapshot, 1, 5), 3);
        // Snapping never moves past the core row.
        assert_eq!(snap_start_row_to_block_boundary(&snapshot, 1, 2), 1);
        // A row that is already a good start after a blank line stays put.
        assert_eq!(snap_start_row_to_block_boundary(&snapshot, 3, 5), 3);
    }

    #[test]
    fn test_snap_start_row_falls_back_to_first_good_start() {
        let snapshot = text_snapshot(concat!(
            "}\n",          // 0
            "let x = 1;\n", // 1
            "let y = 2;\n", // 2
            "core\n",       // 3
        ));
        // No after-blank boundary in the window; the first good start wins.
        assert_eq!(snap_start_row_to_block_boundary(&snapshot, 0, 3), 1);
    }

    #[test]
    fn test_snap_end_row_ends_before_blank_line() {
        let snapshot = text_snapshot(concat!(
            "core\n", // 0
            "a\n",    // 1
            "b\n",    // 2
            "\n",     // 3
            "c\n",    // 4
            "d\n",    // 5
        ));
        // Row 4 is followed by non-blank row 5, so scan back to row 2 which
        // precedes the blank row 3.
        assert_eq!(snap_end_row_to_block_boundary(&snapshot, 4, 0), 2);
        // Snapping never moves before the core row.
        assert_eq!(snap_end_row_to_block_boundary(&snapshot, 4, 4), 4);
        // The last row of the file stays put.
        let max_row = snapshot.max_point().row;
        assert_eq!(
            snap_end_row_to_block_boundary(&snapshot, max_row, 0),
            max_row
        );
    }

    #[test]
    fn test_limit_retrieved_context_keeps_high_priority_snippet_under_tight_budget() {
        let line = "0123456789\n";
        let excerpt = |row_range: Range<u32>, order: usize, context_source: ContextSource| {
            let text = line.repeat((row_range.end - row_range.start) as usize);
            RelatedExcerpt {
                row_range,
                text: text.into(),
                order,
                context_source,
            }
        };
        let related_files = vec![RelatedFile {
            path: Path::new("root/file.rs").into(),
            max_row: 200,
            excerpts: vec![
                excerpt(0..50, 8, ContextSource::GitLog),
                excerpt(50..60, 2, ContextSource::OracleSnippet),
                excerpt(60..200, 8, ContextSource::GitLog),
            ],
            in_open_source_repo: false,
        }];

        // Budget fits the snippet but not the surrounding segments.
        let limited = limit_retrieved_context_to_bytes(&related_files, 20 * line.len());
        assert_eq!(limited.len(), 1);
        assert_eq!(limited[0].excerpts.len(), 1);
        assert_eq!(limited[0].excerpts[0].row_range, 50..60);
        assert_eq!(
            limited[0].excerpts[0].context_source,
            ContextSource::OracleSnippet
        );
    }
}
