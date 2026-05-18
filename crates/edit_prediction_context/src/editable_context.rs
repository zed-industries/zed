use collections::HashMap;
use gpui::{App, AppContext as _, AsyncApp, Entity, EntityId};
use language::{Buffer, BufferSnapshot, Point, ToPoint as _};
use project::{Project, ProjectPath};
use std::{ops::Range, path::Path, sync::Arc};
use text::Anchor;
use util::{paths::PathStyle, rel_path::RelPath};
use zeta_prompt::{ContextSource, RelatedExcerpt, RelatedFile};

use crate::git_log_context::build_git_log_index;

/// This module contains collectors for editable context:
/// excerpts in files that are likely to be edited.

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
    context_sources: Vec<ContextSource>,
    cx: &mut AsyncApp,
) -> anyhow::Result<Vec<RelatedFile>> {
    let mut ranges_by_buffer = RangesByBuffer::default();

    if context_sources.contains(&ContextSource::CurrentFile) {
        collect_current_cursor_context(
            &mut ranges_by_buffer,
            active_buffer.clone(),
            cursor_position,
            cx,
        );
    }
    if context_sources.contains(&ContextSource::EditHistory) {
        collect_edit_history_context(&mut ranges_by_buffer, edit_history, cx);
    }
    if context_sources.contains(&ContextSource::GitLog) {
        collect_git_log_context(&mut ranges_by_buffer, project.clone(), active_buffer, cx).await;
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

fn collect_current_cursor_context(
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
        ContextSource::CurrentFile,
    );
}

fn collect_edit_history_context(
    ranges_by_buffer: &mut RangesByBuffer,
    edit_history: Vec<EditHistoryContextEntry>,
    cx: &mut AsyncApp,
) {
    for (index, entry) in edit_history.into_iter().enumerate() {
        let edit_history_range = entry.buffer.read_with(cx, |buffer, _cx| {
            expanded_anchor_range(
                &buffer.snapshot(),
                entry.edited_range,
                EDIT_HISTORY_CONTEXT_LINE_COUNT,
            )
        });

        push_context_range(
            ranges_by_buffer,
            entry.buffer,
            edit_history_range,
            index + 1,
            ContextSource::EditHistory,
        );
    }
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
        .background_spawn(async move { build_git_log_index(&worktree_abs_path) })
        .await;
    let index = match index_result {
        Ok(index) => index,
        Err(error) => {
            log::debug!("failed to build git log context index: {error:#}");
            return;
        }
    };

    let next_order = ranges_by_buffer
        .values()
        .flat_map(|(_, ranges)| ranges.iter().map(|range| range.order))
        .max()
        .map_or(0, |order| order + 1);

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
            let start = snapshot.anchor_before(Point::new(0, 0));
            let end_row = GIT_LOG_CONTEXT_LINE_COUNT.min(snapshot.max_point().row);
            let end = snapshot.anchor_after(Point::new(end_row, snapshot.line_len(end_row)));
            start..end
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
        ContextSource::CurrentFile => 1,
        ContextSource::EditHistory => 2,
        ContextSource::GitLog => 3,
    }
}
