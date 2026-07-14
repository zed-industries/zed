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
use zeta_prompt::{
    Chunk, ContextFile, ContextSource, Retrieval, default_retrieval_policy_key,
    multi_region::is_good_block_start,
};

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
type RangesByBuffer = HashMap<EntityId, (Entity<Buffer>, Vec<PendingRetrieval>)>;

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

struct PendingRetrieval {
    range: Range<Anchor>,
    rank: usize,
    source: ContextSource,
    score: Option<f32>,
}

struct ResolvedPendingRetrieval {
    range: Range<Point>,
    rank: usize,
    source: ContextSource,
    score: Option<f32>,
}

pub async fn collect_editable_context(
    project: Entity<Project>,
    active_buffer: Entity<Buffer>,
    cursor_position: Anchor,
    edit_history: Vec<EditHistoryContextEntry>,
    oracle_targets: Vec<OracleTarget>,
    context_sources: Vec<ContextSource>,
    cx: &mut AsyncApp,
) -> anyhow::Result<Vec<ContextFile>> {
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
        let mut context_files = ranges_by_buffer
            .into_values()
            .filter_map(|(buffer, ranges)| context_file_for_ranges(&project, &buffer, ranges, cx))
            .collect::<Vec<_>>();
        context_files.sort_by_key(|file| {
            file.retrievals
                .iter()
                .enumerate()
                .map(|(index, retrieval)| default_retrieval_policy_key(0, index, retrieval))
                .min()
                .unwrap_or((usize::MAX, usize::MAX, usize::MAX, usize::MAX))
        });
        context_files
    }))
}

pub fn limit_retrieved_context_to_bytes(
    context_files: &[ContextFile],
    max_bytes: usize,
) -> Vec<ContextFile> {
    struct RetrievalCandidate {
        file_index: usize,
        retrieval_index: usize,
    }

    let mut candidates = context_files
        .iter()
        .enumerate()
        .flat_map(|(file_index, file)| {
            file.retrievals
                .iter()
                .enumerate()
                .map(move |(retrieval_index, _)| RetrievalCandidate {
                    file_index,
                    retrieval_index,
                })
        })
        .collect::<Vec<_>>();
    candidates.sort_by_key(|candidate| {
        let retrieval = &context_files[candidate.file_index].retrievals[candidate.retrieval_index];
        default_retrieval_policy_key(candidate.file_index, candidate.retrieval_index, retrieval)
    });

    let mut covered_ranges_by_file = vec![Vec::<Range<u32>>::new(); context_files.len()];
    let mut selected_bytes: usize = 0;

    for candidate in candidates {
        let file = &context_files[candidate.file_index];
        let retrieval = &file.retrievals[candidate.retrieval_index];
        let added_bytes = uncovered_retrieval_bytes(
            file,
            retrieval,
            &covered_ranges_by_file[candidate.file_index],
        );
        if added_bytes == 0 || selected_bytes.saturating_add(added_bytes) > max_bytes {
            continue;
        }

        selected_bytes += added_bytes;
        push_merged_row_range(
            &mut covered_ranges_by_file[candidate.file_index],
            retrieval.row_range.clone(),
        );
    }

    context_files
        .iter()
        .enumerate()
        .filter_map(|(file_index, file)| {
            let chunks = chunks_for_row_ranges(file, &covered_ranges_by_file[file_index]);
            if chunks.is_empty() {
                return None;
            }

            Some(ContextFile {
                path: file.path.clone(),
                max_row: file.max_row,
                chunks,
                retrievals: file.retrievals.clone(),
                syntax_ranges: file.syntax_ranges.clone(),
            })
        })
        .collect()
}

fn uncovered_retrieval_bytes(
    file: &ContextFile,
    retrieval: &Retrieval,
    covered_ranges: &[Range<u32>],
) -> usize {
    let mut bytes = 0;

    for chunk in &file.chunks {
        let start = retrieval.row_range.start.max(chunk.row_range.start);
        let end = retrieval.row_range.end.min(chunk.row_range.end);
        if start >= end {
            continue;
        }
        for (row, line) in (chunk.row_range.start..).zip(chunk.text.split_inclusive('\n')) {
            if row >= end {
                break;
            }
            if row >= start
                && !covered_ranges
                    .iter()
                    .any(|covered_range| covered_range.contains(&row))
            {
                bytes += line.len();
            }
        }
    }

    bytes
}

fn push_merged_row_range(covered_ranges: &mut Vec<Range<u32>>, range: Range<u32>) {
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
        None,
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
            index,
            ContextSource::EditHistory,
            None,
        );
    }
}

fn collect_edit_history_file_context(
    ranges_by_buffer: &mut RangesByBuffer,
    edit_history: &[EditHistoryContextEntry],
    cx: &mut AsyncApp,
) {
    let mut seen_buffers = HashSet::default();
    let mut index = 0;

    for entry in edit_history {
        if !seen_buffers.insert(entry.buffer.entity_id()) {
            continue;
        }

        collect_full_buffer_context(
            ranges_by_buffer,
            entry.buffer.clone(),
            index,
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
    let candidates = collect_bm25_context(
        project.clone(),
        active_buffer,
        cursor_position,
        edit_history,
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
        candidate.rank,
        ContextSource::Bm25,
        candidate.score,
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
            index,
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
                index,
                ContextSource::OracleSnippet,
                None,
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
    rank: usize,
    source: ContextSource,
    cx: &mut AsyncApp,
) {
    let range = buffer.read_with(cx, |buffer, _cx| full_file_anchor_range(&buffer.snapshot()));
    push_context_range(ranges_by_buffer, buffer, range, rank, source, None);
}

fn full_file_anchor_range(snapshot: &BufferSnapshot) -> Range<Anchor> {
    let start = snapshot.anchor_before(Point::new(0, 0));
    let max_point = snapshot.max_point();
    let end = snapshot.anchor_after(max_point);
    start..end
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

    for (index, related_path) in index
        .get_related(active_path.as_std_path(), GIT_LOG_CONTEXT_FILE_COUNT)
        .into_iter()
        .enumerate()
    {
        let Ok(related_path) = RelPath::new(&related_path, PathStyle::Unix) else {
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
            index,
            ContextSource::GitLog,
            None,
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
    rank: usize,
    source: ContextSource,
    score: Option<f32>,
) {
    ranges_by_buffer
        .entry(buffer.entity_id())
        .or_insert_with(|| (buffer.clone(), Vec::new()))
        .1
        .push(PendingRetrieval {
            range,
            rank,
            source,
            score,
        });
}

fn context_file_for_ranges(
    project: &Project,
    buffer: &Entity<Buffer>,
    ranges: Vec<PendingRetrieval>,
    cx: &App,
) -> Option<ContextFile> {
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

    let retrievals = resolved_context_ranges(ranges, &snapshot)
        .into_iter()
        .filter_map(|range| {
            let row_range = range.range.start.row..range.range.end.row;
            (row_range.start < row_range.end).then(|| Retrieval {
                source: range.source,
                row_range,
                rank: range.rank,
                score: range.score,
            })
        })
        .collect::<Vec<_>>();
    if retrievals.is_empty() {
        return None;
    }

    let mut chunk_row_ranges = Vec::new();
    for retrieval in &retrievals {
        push_merged_row_range(&mut chunk_row_ranges, retrieval.row_range.clone());
    }
    let chunks = chunks_for_snapshot_ranges(&snapshot, &chunk_row_ranges);

    Some(ContextFile {
        path,
        max_row: snapshot.max_point().row,
        chunks,
        retrievals,
        syntax_ranges: Vec::new(),
    })
}

fn chunks_for_row_ranges(file: &ContextFile, row_ranges: &[Range<u32>]) -> Vec<Chunk> {
    row_ranges
        .iter()
        .filter_map(|row_range| {
            let mut text = String::new();
            for chunk in &file.chunks {
                let start = row_range.start.max(chunk.row_range.start);
                let end = row_range.end.min(chunk.row_range.end);
                if start >= end {
                    continue;
                }
                for (row, line) in (chunk.row_range.start..).zip(chunk.text.split_inclusive('\n')) {
                    if row >= end {
                        break;
                    }
                    if row >= start {
                        text.push_str(line);
                    }
                }
            }
            (!text.is_empty()).then(|| Chunk {
                row_range: row_range.clone(),
                text: text.into(),
            })
        })
        .collect()
}

fn chunks_for_snapshot_ranges(snapshot: &BufferSnapshot, row_ranges: &[Range<u32>]) -> Vec<Chunk> {
    row_ranges
        .iter()
        .filter_map(|row_range| {
            if row_range.start >= row_range.end {
                return None;
            }
            let start = Point::new(row_range.start, 0);
            let end = if row_range.end > snapshot.max_point().row {
                snapshot.max_point()
            } else {
                Point::new(row_range.end, 0)
            };
            let text = snapshot.text_for_range(start..end).collect::<String>();
            (!text.is_empty()).then(|| Chunk {
                row_range: row_range.clone(),
                text: text.into(),
            })
        })
        .collect()
}

fn resolved_context_ranges(
    ranges: Vec<PendingRetrieval>,
    snapshot: &BufferSnapshot,
) -> Vec<ResolvedPendingRetrieval> {
    ranges
        .into_iter()
        .filter_map(|range| {
            let start = range.range.start.to_point(snapshot);
            let end = range.range.end.to_point(snapshot);
            if start >= end {
                return None;
            }

            Some(ResolvedPendingRetrieval {
                range: start..end,
                rank: range.rank,
                source: range.source,
                score: range.score,
            })
        })
        .collect()
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
        let chunk = |row_range: Range<u32>| Chunk {
            text: line
                .repeat((row_range.end - row_range.start) as usize)
                .into(),
            row_range,
        };
        let retrieval = |row_range: Range<u32>, rank: usize, source: ContextSource| Retrieval {
            source,
            row_range,
            rank,
            score: None,
        };
        let context_files = vec![ContextFile {
            path: Path::new("root/file.rs").into(),
            max_row: 200,
            chunks: vec![chunk(0..200)],
            retrievals: vec![
                retrieval(0..50, 8, ContextSource::GitLog),
                retrieval(50..60, 2, ContextSource::OracleSnippet),
                retrieval(60..200, 8, ContextSource::GitLog),
            ],
            syntax_ranges: Vec::new(),
        }];

        let limited = limit_retrieved_context_to_bytes(&context_files, 20 * line.len());
        assert_eq!(
            limited,
            vec![ContextFile {
                path: Path::new("root/file.rs").into(),
                max_row: 200,
                chunks: vec![Chunk {
                    row_range: 50..60,
                    text: line.repeat(10).into(),
                }],
                retrievals: vec![
                    Retrieval {
                        source: ContextSource::GitLog,
                        row_range: 0..50,
                        rank: 8,
                        score: None,
                    },
                    Retrieval {
                        source: ContextSource::OracleSnippet,
                        row_range: 50..60,
                        rank: 2,
                        score: None,
                    },
                    Retrieval {
                        source: ContextSource::GitLog,
                        row_range: 60..200,
                        rank: 8,
                        score: None,
                    },
                ],
                syntax_ranges: Vec::new(),
            }]
        );
    }
}
