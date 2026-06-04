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
use zeta_prompt::{ContextSource, RelatedExcerpt, RelatedFile};

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

type RangesByBuffer = HashMap<EntityId, (Entity<Buffer>, Vec<EditableContextRange>)>;

#[derive(Clone)]
pub struct EditHistoryContextEntry {
    pub buffer: Entity<Buffer>,
    pub edited_range: Range<Anchor>,
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
    oracle_paths: Vec<Arc<Path>>,
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
        collect_oracle_file_context(&mut ranges_by_buffer, project.clone(), oracle_paths, cx).await;
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
    oracle_paths: Vec<Arc<Path>>,
    cx: &mut AsyncApp,
) {
    let next_order = next_context_order(ranges_by_buffer);
    let mut seen_buffers = HashSet::default();
    let mut index = 0;

    for path in oracle_paths {
        let buffer = match open_buffer_for_path(&project, &path, cx).await {
            Ok(Some(buffer)) => buffer,
            Ok(None) => {
                log::debug!("failed to find oracle file path: {}", path.display());
                continue;
            }
            Err(error) => {
                log::debug!(
                    "failed to open oracle file path {}: {error:#}",
                    path.display()
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
    let start = snapshot.anchor_before(Point::new(start_row, 0));
    let end = snapshot.anchor_after(Point::new(end_row, snapshot.line_len(end_row)));
    start..end
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

    let ranges = resolved_context_ranges(ranges, &snapshot);

    let mut excerpts = ranges
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
    excerpts.sort_by_key(|excerpt| excerpt.order);

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

#[allow(dead_code)]
fn merge_overlapping_ranges(ranges: &mut Vec<ResolvedEditableContextRange>) {
    ranges.sort_by_key(|range| (range.range.start, range.range.end));
    let mut merged: Vec<ResolvedEditableContextRange> = Vec::new();

    for range in ranges.drain(..) {
        if let Some(last_range) = merged.last_mut()
            && range.range.start <= last_range.range.end
        {
            if context_source_order(range.context_source)
                < context_source_order(last_range.context_source)
            {
                last_range.context_source = range.context_source;
            }
            last_range.range.end = last_range.range.end.max(range.range.end);
            last_range.order = last_range.order.min(range.order);
            continue;
        }

        merged.push(range);
    }

    *ranges = merged;
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
        ContextSource::OracleFile => 7,
    }
}
